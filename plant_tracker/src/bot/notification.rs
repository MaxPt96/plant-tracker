use teloxide::prelude::Requester;
use teloxide::types::ChatId;

use crate::bot::handlers::plants::get_all_plants_status;

use crate::bot::keyboards::back_to;
use crate::{db_operations, prelude::*};

/// Sends watering reminders to all users who have plants that need urgent attention.
///
/// Called daily at 09:00 by [`notification_loop`].
///
/// For each registered user, fetches the watering status of all their plants
/// and sends a notification if any plant is in one of the critical states:
/// - `"Полив просрочен"` — watering is overdue
/// - `"Полить сегодня"` — watering is due today
///
/// Errors per user are logged to stderr and skipped — one failing user
/// does not interrupt notifications for others.
pub async fn chat_notification(bot: &Bot, pool: &PgPool, users: Vec<i64>) {
    for id in users {
        if let Err(e) = notify_user(bot, pool, id).await {
            tracing::error!("user {id}: {e:?}");
        }
    }
}


async fn notify_user(bot: &Bot, pool: &PgPool, id: i64) -> anyhow::Result<()> {
    let chat_id = ChatId(id);
    let status = get_all_plants_status(pool, id).await?;

    if !(status.contains("Полив просрочен") || status.contains("Полить сегодня")) {
        return Ok(());
    }

    // Удаление старого сообщения не критично
    if let Ok(msg_id) = db_operations::get_current_msg_id(pool, id).await {
        if let Err(e) = bot
            .delete_message(chat_id, MessageId(msg_id as i32))
            .await
        {
            tracing::warn!("delete msg for {id}: {e}");
        }
    }

    let res = bot
        .send_message(chat_id, format!("💧 Напоминание о поливе:\n{}", status))
        .reply_markup(back_to())
        .await?;

    // Сразу после отправки, чтобы не было дублей
    db_operations::update_last_notification_time(pool, id).await?;
    db_operations::save_current_msg_id(pool, id, res.id.0.into()).await?;

    Ok(())
}
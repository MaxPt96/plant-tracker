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
pub async fn chat_notification(bot: &Bot, pool: &PgPool) -> anyhow::Result<()> {
    let users = db_operations::get_all_users(pool)
        .await
        .context("failed to get user")?;

    for user in users {
        let chat_id = ChatId(user.id);

        let status = match get_all_plants_status(pool, user.id).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to get status for {}: {e}", user.id);
                continue;
            }
        };
        if status.contains("Полив просрочен") || status.contains("Полить сегодня")
        {
            let msg_id = db_operations::get_current_msg_id(pool, chat_id.0)
                .await
                .context("Failed to get msg_id")?;

            tracing::info!("{msg_id}");

            let delete_prev_msg = bot.delete_message(chat_id, MessageId(msg_id as i32)).await;

            if delete_prev_msg.is_ok() {
                tracing::info!("Message was deleted successfully.");
            } else {
                tracing::warn!("Failed to delete message for {}.", chat_id);
            }

            let res = bot
                .send_message(chat_id, format!("💧 Напоминание о поливе:\n{}", status))
                .reply_markup(back_to())
                .await?;

            db_operations::save_current_msg_id(pool, chat_id.0, res.id.0.into()).await?;
        }
    }

    Ok(())
}

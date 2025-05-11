use crate::dado::{processar_dado};
use poise::serenity_prelude::{Context, CreateMessage, EventHandler, Message, MessageReference};

pub(crate) struct Handler;

///Evento do dado
#[poise::async_trait]
impl EventHandler for Handler {
    async fn message(
        &self,
        context: Context,
        msg: Message,
    ) {
        if !msg.author.bot {

            let msgdb = msg.clone();
            let escrita: String = msg.content;
            let resposta = MessageReference::from((msgdb.channel_id,msgdb.id));

            let data: &str = escrita.as_str();

            let cont = processar_dado(data);
            
            if cont == ""{
                return;
            }

            let bob = CreateMessage::new().content(cont).reference_message(resposta);
            msg.channel_id.send_message(context, bob).await.expect("OH NO");
            
        }
    }
}
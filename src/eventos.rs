use crate::dado::{rolar_dado, rolar_dados};
use poise::serenity_prelude::{Context, CreateMessage, EventHandler, Message, MessageReference};
use regex::Regex;

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


            let splitter = Regex::new(r"^(\d*)#?(\d*)[Dd](\d+)(\s*[+\-*/]\s*\d+(\.\d+)?(?:\s*[+\-*/]\s*\d+(\.\d+)?)*)?(?:\s*(=|>=|<=|!=|<|>)\s*(\d+))?$").unwrap();

            if let Some(usos) = splitter.captures(data) {
                let dados = usos[1].parse::<u32>().unwrap_or(1);
                let quant = usos[2].parse::<u32>().unwrap_or(1);
                let lados = usos[3].parse::<u32>().unwrap_or(8);
                let modif = usos.get(4).map(|m| m.as_str()).unwrap_or("");
                let resp: String;
                let compare = usos.get(7).map(|m| m.as_str()).unwrap_or(""); // Operador de comparação
                let compai = usos.get(8).map(|m| m.as_str().parse::<u32>().unwrap_or(0)); // Número de comparação
                let compa = compai.unwrap_or(0);

                if data.contains('#') {
                    resp = rolar_dados(dados, lados, quant, modif,1, compare,&compa);
                } else {
                    resp = rolar_dado(&lados, &dados, modif,&1, compare,&compa);
                }

                let bob = CreateMessage::new().content(resp).reference_message(resposta);
                msg.channel_id.send_message(context, bob).await.expect("OH NO");
            };
        }
    }
}
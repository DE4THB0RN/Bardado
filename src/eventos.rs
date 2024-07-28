use calc_engine::calculate;
use poise::serenity_prelude::{Context, CreateMessage, EventHandler, Message, MessageReference};
use rand::Rng;
use regex::Regex;


pub(crate) struct Handler;

fn pega_formula(modifier : String) -> u32 {

    let formula : &str = modifier.as_str();
    let sum : f64 = calculate(formula).unwrap();
    let res : u32 = sum.round() as u32;

    res
}

fn rolar_dado(lados : &u32,vezes : &u32, modif : &str) -> String {

    let xlados : u32 = *lados;
    let xvezes : u32 = *vezes;

    let mut rng = rand::thread_rng();

    let mut x : u32;
    let mut maximum : u32 = 0;
    for _ in 0..xvezes{
        maximum += xlados;
    }
    let mut inst : String;
    let mut total : u32 = 0;
    let mut tudo : String = String::from("[");
    for i in 0..xvezes{

        x = rng.gen_range(1..=xlados);
        inst = x.to_string();

        if x >= xlados - 1 {
            tudo.push_str(" **");
            tudo.push_str(&inst);
            tudo.push_str("** ");
        }
        else if x <= xvezes + 1 && xlados >= 4 {
            tudo.push(' ');
            tudo.push('*');
            tudo.push_str(&inst);
            tudo.push('*');
            tudo.push(' ');
        }
        else{
            tudo.push(' ');
            tudo.push_str(&inst);
            tudo.push(' ');
        }

        if xvezes >= 2 && i <= xvezes - 2 {
            tudo.push(',');
        }

        total += x;
    }
    tudo.push(']');
    tudo.push('→');

    let full_form = format!("{total}{modif}");
    let mut res : u32 = total;
    if !modif.is_empty(){
        res = pega_formula(full_form)
    }

    inst = res.to_string();

    if total >= maximum - 2{
        tudo.push_str(" **");
        tudo.push_str(&inst);
        tudo.push_str("** ");
        tudo.push_str(" **CRITIQUEI**");
    }
    else if total <= vezes + 1 && xlados >= 7 {
        tudo.push(' ');
        tudo.push('*');
        tudo.push_str(&inst);
        tudo.push('*');
        tudo.push(' ');
        tudo.push_str(" ou nou");
    }
    else{
        tudo.push(' ');
        tudo.push_str(&inst);
    }

    tudo
}

fn rolar_dados(dados : u32,lados : u32,vezes : u32, modif : &str) -> String{
    let mut resp : String = String::from("");

    println!("dados: {} lados: {} vezes: {} modif: {}",dados,lados,vezes,modif);

    for _ in 0..dados{
        let vez : String = rolar_dado(&lados,&vezes,modif);
        resp.push_str(vez.as_str());
        resp.push('\n');
    }

    resp
}

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

            let splitter = Regex::new(r"^(\d*)#?(\d*)d(\d+)([+/*-]?.+)?$").unwrap();

            if let Some(usos) = splitter.captures(data) {
                let dados = usos[1].parse::<u32>().unwrap_or(1);
                let quant = usos[2].parse::<u32>().unwrap_or(1);
                let lados = usos[3].parse::<u32>().unwrap_or(8);
                let modif = usos.get(4).map(|m| m.as_str()).unwrap_or("");
                let resp: String;

                if data.contains('#') {
                    resp = rolar_dados(dados, lados, quant, modif);
                } else {
                    resp = rolar_dado(&lados, &dados, modif);
                }

                let bob = CreateMessage::new().content(resp).reference_message(resposta);
                msg.channel_id.send_message(context, bob).await.expect("OH NO");
            };
        }
    }
}
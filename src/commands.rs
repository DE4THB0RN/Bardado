use rand::Rng;
use crate::{statuses, Context, Error};
use crate::dado::{dado_iniciativa, rolar_dados};

/// Responds with "world!"
#[poise::command(slash_command,prefix_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

///O limite mínimo dele é 0 aqui
#[poise::command(slash_command,prefix_command)]
pub async fn dad0(
    ctx: Context<'_>,
    #[description = "Número de jogadas ex: 5 XdX"] mut jogadas : u32,
    #[description = "Número de dados ex: 2dX"] mut dados : u32,
    #[description = "Número de lados ex: d20"] lados : u32,
    #[description = "Operação a ser realizada ex: dX + 5"] mut operation : String) -> Result<(), Error> {

    if jogadas == 0{
        jogadas = 1;
    }

    if dados == 0{
        dados = 1;
    }

    if ! operation.starts_with('+') || operation.starts_with('-') || operation.starts_with('/') || operation.starts_with('*'){
        operation = "+".to_string() + &operation;
    }

    let resp = rolar_dados(jogadas, lados, dados, operation.as_str(),0,"",&0).to_string();

    ctx.say(resp).await?;

    Ok(())
}

///Taca iniciativa aí
#[poise::command(slash_command,prefix_command)]
pub async fn iniciativa (
    ctx: Context<'_>,
    #[description = "Valor de iniciativa na sua ficha"] bonus : u32,
    #[description = "Nome do personagem"] persona : String
) -> Result<(), Error> {

    let x = dado_iniciativa(bonus);
    {
        let mudar = &mut statuses::INICIA_GERAL.lock().unwrap().inis;

        mudar.insert(persona,x.0);
    }

    ctx.say(x.1).await?;

    Ok(())
}

///Atualiza a lista de iniciativa
#[poise::command(slash_command,prefix_command)]
pub async fn mudar_iniciativa(
    ctx: Context<'_>,
    #[description = "Valor de iniciativa"] valor : u32,
    #[description = "Nome do personagem"] persona : String
) -> Result<(), Error>{

    let per = persona.clone();
    {
        let mudar = &mut statuses::INICIA_GERAL.lock().unwrap().inis;

        mudar.insert(persona,valor);
    }

    let mut resposta = String::new();

    resposta.push_str("Iniciativa ");
    resposta.push_str(per.as_str());
    resposta.push_str(": ");
    resposta.push_str(valor.to_string().as_str());

    ctx.say(resposta).await?;


    Ok(())
}

///Limpa a lista de iniciativa
#[poise::command(slash_command,prefix_command)]
pub async fn limpar_iniciativa (
    ctx: Context<'_>,) -> Result<(), Error>{

    {
        let mudar = &mut statuses::INICIA_GERAL.lock().unwrap().inis;

        mudar.clear();
    }

    ctx.say("Apaguei as iniciativas").await?;


    Ok(())
}

///Mostra a lista de iniciativa
#[poise::command(slash_command,prefix_command)]
pub async fn listar_iniciativa (
    ctx: Context<'_>,
) -> Result<(), Error>{


    {
        if statuses::INICIA_GERAL.lock().unwrap().inis.is_empty(){
            ctx.say("A lista está vazia").await?;
            return  Ok(())
        }
    }

    let sorter_data: Vec<(String, u32)> = {
        let inicia_geral = statuses::INICIA_GERAL.lock().unwrap();
        let mut sorter: Vec<(&String,&u32)> = inicia_geral.inis.iter().collect();


        sorter.sort_by(|a, b| b.1.cmp(a.1));

        sorter.into_iter().map(|(k, v)| (k.clone(), *v)).collect()
    }; 

    let mut resposta : String = String::new();
    let mut x : u32 = 1;
    for item in sorter_data{
        
        resposta.push_str(x.to_string().as_str());
        resposta.push_str(" -> ");
        resposta.push_str(item.0.as_str());
        resposta.push_str(": ");
        resposta.push_str(item.1.to_string().as_str());
        
        resposta.push('\n');
        
        x += 1;
    }
    
    ctx.say(resposta).await?;

    Ok(())
}

///Aqui o link do Kanka
#[poise::command(slash_command,prefix_command)]
pub async fn kanka (
    ctx: Context<'_>,
) -> Result<(), Error>{

    let mut extra = String::new();


    if ctx.author().id == 442739973960237057 || ctx.author().id == 1064209850483093655{
        extra.push('\n');
        extra.push_str("2000, só leia em dia de RPG");
    }


    let tem_extra = {
        let mut rng = rand::rng();
        let x : u32 = rng.random_range(1..=20);
        let mut resposta = false;
        if x == 20 {
            resposta = true;
        }
        resposta
    };

    if tem_extra {
        extra.push('\n');
        extra.push_str("NÃO SE ESQUEÇA DE LER CLASSE ESPECIAL I E II LINK AQUI NO CANAL <#893280249004060702>");
    }

    let mut resposta = String::from("Aqui o link do Kanka: https://app.kanka.io/w/285561");

    if !extra.is_empty(){
        resposta.push_str(extra.as_str());
    }

    ctx.say(resposta).await?;

    Ok(())
}

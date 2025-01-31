use crate::{Context, Error};
use crate::dado::rolar_dados;

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

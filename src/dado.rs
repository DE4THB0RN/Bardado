use calc_engine::calculate;
use rand::Rng;
use regex::{Regex};

fn pega_formula(modifier : String) -> u32 {

    let formula : &str = modifier.as_str();
    let sum : f64 = calculate(formula).unwrap();
    let res : u32 = sum.round() as u32;

    res
}

fn compara_resultado(compare : &str, compa : u32, resi : u32) -> bool{
    let mut res : bool = false;
    let compai = compa.clone();

    if compare == "="{
        res = compai == resi;
    }
    else if compare == "<"{
        res = compai > resi;
    }
    else if compare == ">"{
        res = compai < resi;
    }
    else if compare == "<="{
        res = compai >= resi;
    }
    else if compare == ">="{
        res = compai <= resi;
    }

    res
}

pub fn rolar_dado(lados : &u32,vezes : &u32, modif : &str, min : &u32, compare : &str, compa : &u32) -> String {

    let xlados : u32 = *lados;
    let xvezes : u32 = *vezes;
    let minimum : u32 = *min;
    let compi = compa.clone();
    let mut rng = rand::rng();

    let mut x : u32;
    let mut maximum : u32 = 0;
    for _ in 0..xvezes{
        maximum += xlados;
    }
    let mut inst : String;
    let mut total : u32 = 0;
    let mut tudo : String = String::from("[");
    for i in 0..xvezes{

        x = rng.random_range(minimum..=xlados);
        inst = x.to_string();

        if x >= xlados - 1 {
            tudo.push_str(" **");
            tudo.push_str(&inst);
            tudo.push_str("** ");
        }
        else if x <= minimum + 1 && xlados >= 4 {
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

    let mut comp : String = "".to_string();

    if !compare.is_empty(){
        let resi = res.clone();
        if compara_resultado(compare, compi, resi){
            comp = "**Sucesso**".to_string();
        }
        else{
            comp = "*falha...*".to_string();
        }
    }

    inst = res.to_string();


    if maximum > 6 && total >= maximum - 2{
        tudo.push_str(" **");
        tudo.push_str(&inst);
        tudo.push_str("** ");
        tudo.push_str(" **CRITIQUEI**");
        if !comp.is_empty(){
            tudo.push_str(" e ");
        }
    }
    else if maximum < 7 && total == maximum {
        tudo.push_str(" **");
        tudo.push_str(&inst);
        tudo.push_str("** ");
        tudo.push_str(" **CRITIQUEI**");
        if !comp.is_empty(){
            tudo.push_str(" e ");
        }
    }
    else if total <= vezes + 1 && xlados >= 7 {
        tudo.push(' ');
        tudo.push('*');
        tudo.push_str(&inst);
        tudo.push('*');
        tudo.push(' ');
        tudo.push_str(" ou nou");
        if !comp.is_empty(){
            tudo.push_str(", foi ");
        }
    }
    else{
        tudo.push(' ');
        tudo.push_str(&inst);
        if !comp.is_empty(){
            tudo.push_str(" ");
        }
    }
    
    tudo.push_str(&comp);

    tudo
}

pub fn rolar_dados(dados : u32,lados : u32,vezes : u32, modif : &str, min :u32, compare : &str, compa : &u32) -> String{
    let mut resp : String = String::from("");

   // println!("dados: {} lados: {} vezes: {} modif: {}",dados,lados,vezes,modif);

    for _ in 0..dados{
        let vez : String = rolar_dado(&lados,&vezes,modif,&min,compare,compa);
        resp.push_str(vez.as_str());
        resp.push('\n');
    }

    resp
}

pub fn dado_iniciativa(bonux : u32) -> (u32,String){

    let mut rng = rand::rng();
    let x : u32;
    x = rng.random_range(1..=8);
    let total = x + bonux;
    
    let mut resposta = String::from("[");
    let inst : String = x.to_string();
    let tot = total.to_string();

    if x >= 7 {
        resposta.push_str(" **");
        resposta.push_str(&inst);
        resposta.push_str("** ");
    }
    else if x <= 2 {
        resposta.push(' ');
        resposta.push('*');
        resposta.push_str(&inst);
        resposta.push('*');
        resposta.push(' ');
    }
    else{
        resposta.push(' ');
        resposta.push_str(&inst);
        resposta.push(' ');
    }

    resposta.push(']');
    resposta.push('→');

    if  x >= 7{
        resposta.push_str(" **");
        resposta.push_str(&tot);
        resposta.push_str("** ");
        resposta.push_str(" **CRITIQUEI**");
    }
    else if x <= 2{
        resposta.push(' ');
        resposta.push('*');
        resposta.push_str(&tot);
        resposta.push('*');
        resposta.push(' ');
        resposta.push_str(" ou nou");
    }
    else{
        resposta.push(' ');
        resposta.push_str(&tot);
    }
    
    (total,resposta)
}

pub fn processar_dado(data: &str) -> String{

    let splitter = Regex::new(r"(\d*)#?(\d*)[Dd](\d+)(\s*[+\-*/]\s*\d+(\.\d+)?(?:\s*[+\-*/]\s*\d+(\.\d+)?)*)?(?:\s*(=|>=|<=|!=|<|>)\s*(\d+))?").unwrap();

    let mut resp: String = String::new()    ;


    for usos in splitter.captures_iter(data) {
        let dados = usos[1].parse::<u32>().unwrap_or(1);
        let quant = usos[2].parse::<u32>().unwrap_or(1);
        let lados = usos[3].parse::<u32>().unwrap_or(8);
        let modif = usos.get(4).map(|m| m.as_str()).unwrap_or("");

        let compare = usos.get(7).map(|m| m.as_str()).unwrap_or(""); // Operador de comparação
        let compai = usos.get(8).map(|m| m.as_str().parse::<u32>().unwrap_or(0)); // Número de comparação
        let compa = compai.unwrap_or(0);

        if data.contains('#') {
            resp.push_str(rolar_dados(dados, lados, quant, modif,1, compare,&compa).as_str());
        } else {
            resp.push_str(rolar_dado(&lados, &dados, modif,&1, compare,&compa).as_str());
        }
        resp.push('\n');
    }

    if !resp.is_empty(){
        return resp;
    }

    "".to_string()
}
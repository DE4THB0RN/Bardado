use calc_engine::calculate;
use rand::Rng;

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
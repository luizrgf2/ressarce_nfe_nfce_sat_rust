use walkdir::WalkDir;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::path::Path;

fn name_file(dir:&String) ->String{

    let nome = dir[dir.len()-48..dir.len()].to_string();

    nome


}

fn create_dirs(mes: &String, ano: &String){


    let path_atual = format!("./organizado/");
    
    if !Path::new(&path_atual).exists(){

        fs::create_dir_all(&path_atual).unwrap();

    }

    let path_atual = format!("./organizado/{}",&ano);
    
    if !Path::new(&path_atual).exists(){

        fs::create_dir_all(&path_atual).unwrap();

    }

    let path_atual = format!("./organizado/{}/{} {}",&ano,&mes,&ano);
    
    if !Path::new(&path_atual).exists(){

        fs::create_dir_all(&path_atual).unwrap();

    }

    
    let path_atual = format!("./organizado/{}/{} {}/entradas",&ano,&mes,&ano);
    
    if !Path::new(&path_atual).exists(){

        fs::create_dir_all(&path_atual).unwrap();

    }


    let path_atual = format!("./organizado/{}/{} {}/saídas",&ano,&mes,&ano);
    
    if !Path::new(&path_atual).exists(){

        fs::create_dir_all(&path_atual).unwrap();

    }

}

fn verificando_entry(cnpj_emit:&String, cnpj_empresa:&str, tpnf:&str) -> String{

    let mut entry = String::from("");

    let verificando_cnpj = if cnpj_emit == cnpj_empresa{true}else{false};

    if verificando_cnpj{

        if tpnf == "0"{
            entry = "0".to_string();
        }else if tpnf == "1"{
            entry = "1".to_string();
        }else {
            entry = "1".to_string();
        }

    }else{

        if tpnf == "0"{
            entry = "1".to_string();
        }else if tpnf == "1"{
            entry = "0".to_string();
        }else {
            entry = "1".to_string();
        }
    }

    entry



}

fn pegar_valor_no_texto(mut texto : &str, chave:String, chave2:String) -> String{

    
    let mut continuar = true;

    if chave != chave2{

        let tmp :Vec<&str> = texto.split(&chave).collect();


        if tmp.len() == 1{
            return String::from("");
        }

        texto= tmp[1];

        

    }
    


    if texto.contains(&format!("</{}>",chave2)) == false{
        continuar = false;
    } 


    let mut texto_final = "".to_string();

    if continuar == true{

        let  result: Vec<&str> = texto.split(&chave).collect();
        texto_final = if texto.contains(&chave){result[1][1..result[1].len()-2].to_string()}else{"".to_string()};
        let  result: Vec<&str> = texto.split(&chave2).collect();
        texto_final = if texto.contains(&chave2){result[1][1..result[1].len()-2].to_string()}else{"".to_string()};
    };


  
    if texto_final.len() != 0 {

        loop {
            let valor :f32 =match texto_final.parse(){
                
                Ok(file)=>{
                    

                    if chave == "qCom" || chave == "vUnCom"{
                        texto_final = format!("{:.4}",&file).replace(".", ",");
                    }else if texto_final.contains("."){
                        texto_final = format!("{:.2}",&file).replace(".", ",");

                    };

                    //texto_final =texto_final.replace(".", ",");

                    

                    file

                    
                },
                Err(_)=>break



            };

            break;
        }


    };
    
    
    texto_final


}

fn pegar_data_xml(texto :&String) -> (String,String){

    let mut mes = String::from("");
    let mut ano = String::from("");


    let mut result = (mes,ano);
    
    
    let mut data = pegar_valor_no_texto(&texto, "dhSaiEnt".to_string(),"dhSaiEnt".to_string());
    
    
    if data == ""{
        
        data = pegar_valor_no_texto(&texto, "dSaiEnt".to_string(),"dSaiEnt".to_string());

    }
    
    if data == ""{
        
        data = pegar_valor_no_texto(&texto, "dEmi".to_string(),"dEmi".to_string());

    }
    
    if data == ""{


        
        data = pegar_valor_no_texto(&texto, "dhEmi".to_string(),"dhEmi".to_string());


        ano = data[0..4].to_string();
        mes = data[4..6].to_string();


        if mes.contains("-"){

            mes = data[5..7].to_string()

        }



    }else{

        let temp = data.split("-").collect::<Vec<&str>>();

        ano =temp[0].to_string();
        mes = temp[1].to_string();


    }

    result = (mes,ano);

    result

}

pub fn abrir_xml(path : String, cnpj_empresa :String){


    println!("{}",&path);

    let file = File::open(&path).unwrap();
    let mut buferead = BufReader::new(file);
    let mut texto = String::from("");//texto do arquivo xml
    buferead.read_to_string(&mut texto).unwrap();

    //pegando data
    let data = pegar_data_xml(&texto);

    let mes = data.0;
    let ano = data.1;
    
    let cnpj_emit = pegar_valor_no_texto(&texto, "<emit".to_string(), "CNPJ".to_string()); //cnpj emitente dentro do xml
    let tpnf = pegar_valor_no_texto(&texto, "<ide".to_string() , "tpNF".to_string()); //tpnf  dentro do xml


    let entry = verificando_entry(&cnpj_emit, &cnpj_empresa, &tpnf); //arquivo entrada ou saida
    
    create_dirs(&mes, &ano); //criando os diretorios


    //pegando diretorio para salvar o xml final
    let mut path_final = format!("./organizado/{}/{} {}",&ano,&mes,&ano);

    let name_file = name_file(&path);


    if entry == "0"{
        path_final = format!("{}/{}/{}",&path_final,"entradas",&name_file);
    }else if entry == "1"{
        path_final = format!("{}/{}/{}",&path_final,"saídas",&name_file);
    };



    escrever_no_xml(&path_final, texto);

    









    
    


}

pub fn pegar_dirs(){

    let mut cnpj_emit = String::from("");
    
    println!("Digite o cnpj da empresa :");
    
    stdin().read_line(&mut cnpj_emit).unwrap();

    let dirs = WalkDir::new("./arquivos");


    for dir in dirs{

        let dir_atual = &dir.unwrap().path().display().to_string(); //diretorio atual


        if dir_atual.contains(".xml"){


            abrir_xml(dir_atual.to_string(), cnpj_emit.clone().replace("\r", "").replace("\n", ""));


        }else if dir_atual.contains(".txt"){

            abrir_sped(dir_atual);

        };



    }

}

fn escrever_no_xml(path : &String,texto:String){

    let mut arquivo = fs::File::create(path).unwrap();

    arquivo.write_all(texto.as_bytes()).unwrap();

    
   

}

fn abrir_sped(path:&String){

    println!("sped dir {}",path);


    let file = File::open(path).unwrap();
    let mut buferread = BufReader::new(file);
    let mut bytes = Vec::new();
    buferread.read_to_end(&mut bytes).unwrap();
    let texto = format!("{}",String::from_utf8_lossy(&bytes));



    let data = texto.split("\n").collect::<Vec<&str>>()[0].split("|").collect::<Vec<&str>>()[4].to_string();
    let mes = &data[2..4].to_string();
    let ano = &data[4..].to_string();

    create_dirs(&mes, &ano);
    
    let path_padrao = format!("organizado/{}/{} {}/sped.txt",ano,mes,ano);

    escrever_no_xml(&path_padrao, texto);





}
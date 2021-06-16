mod diretorio;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn pegar_chave(linha :&str) -> String{


    let verificafor_nfe_entrada = linha.contains("|C100|0|");
    let verificafor_nfe_saida = linha.contains("|C100|1|");
    let verificafor_sat = linha.contains("|C800|59|");
    let mut chave = String::from("");


    if verificafor_nfe_entrada{


        let lista_tmp :Vec<&str>= linha.split("|").collect();
        chave = lista_tmp[9].to_string();
        

    }else if verificafor_nfe_saida{

        let lista_tmp :Vec<&str>= linha.split("|").collect();
        chave = lista_tmp[9].to_string();
        

    }else if verificafor_sat{

        let lista_tmp :Vec<&str>= linha.split("|").collect();
        chave = lista_tmp[11].to_string();
        
    };

    chave

}

fn verificar_criar_dir(path:&str){

    let path_atual = Path::new(path).exists();

    if !path_atual{

        fs::create_dir(path).unwrap();

    }

}

fn ler_arquivo(path:&String) -> Vec<String>{


    let mut linhas = Vec::new();//variavel amazenara as linhas do sped

    let file = fs::File::open(path).unwrap();

    let mut buferread = BufReader::new(file);

    let mut buffer = Vec::new();

    buferread.read_to_end(&mut buffer).unwrap();

    let texto = String::from_utf8_lossy(&buffer);

    if texto.len() != 0{

        for linha in texto.lines(){
            
            if linha.contains("|C100|0|") == true || linha.contains("|C100|1|") == true || linha.contains("|C800|59|") == true{

                linhas.push(linha.to_string());
            }
            
    
        }

    }else {
        
        let mut texto = String::new();
        buferread.read_to_string(&mut texto).unwrap();

        let linhas_tmp: Vec<&str> = texto.split("\n").collect();

        for linha in linhas_tmp{

            if linha.contains("|C100|0|") == true || linha.contains("|C100|1|") == true || linha.contains("|C800|59|") == true{
                
                linhas.push(linha.to_string());

            }

        }

    }


    linhas



}

pub fn ler_sped(){

    let diretorios = diretorio::pegar_xmls_speds();

    let dirs_sped = diretorios.speds;
    let dirs_xmls = diretorios.xmls;



    for dir_sped in dirs_sped{




        
        

        //abrindo o sped.txt
        let linhas_sped = ler_arquivo(&dir_sped);
        //analisando linha por linha do sped
        for linha in linhas_sped{

            //abrindo xmls


            let linha = &linha;
            for dir_xml in &dirs_xmls{

                if dir_xml.contains(&pegar_chave(linha)){

                    organizar(dir_xml.to_string(), &linha,&dir_sped)


                }

            }

        }





    }

}

fn mover_sped(sped_dir: &String, path_atual: String){

    let mut linhas = String::new();

    let file_atual = fs::File::open(sped_dir).unwrap();

    let mut buferread = BufReader::new(file_atual);

    let mut buffer = Vec::new();


    buferread.read_to_end(&mut buffer).unwrap();

    let texto = String::from_utf8_lossy(&buffer);

    for linha in texto.lines(){


        if linhas.len() == 0{

            linhas = linha.to_string();
            
        }else{
            
            linhas.push_str(&format!("\n{}",linha.to_string()));

        }

    }


    let mut file_atual  = fs::File::create(format!("{}/{}",path_atual,"sped.txt")).unwrap();

    file_atual.write_all(linhas.as_bytes()).unwrap();

}

fn organizar(dir_xml:String,line_sped:&String, dir_sped: &String){

    let verificador_nfe_entrada = line_sped.contains("|C100|0|");
    let verificador_nfe_saida = line_sped.contains("|C100|1|");
    let verificador_cfe = line_sped.contains("|C800|59|");



    if verificador_nfe_entrada{


        let line_atual:Vec<&str> = line_sped.split("|").collect();

        let chave = pegar_chave(&line_sped);

        if chave.len() != 0 {
        let tipo_doc = line_atual[5];

        if tipo_doc == "55" || tipo_doc == "65"{


            let mes_ano =  &format!("{}_{}",&line_atual[11][2..4],&line_atual[11][4..8])[..];

            let ano = &line_atual[11][4..];




                let  path_padrao = format!("{}/{}","./organizado",ano);
                
                criar_diretorios(mes_ano, ano,&dir_sped);
                
                let  dir_entrada = format!("{}/{}/{}",&path_padrao,mes_ano,"entrada");
                let  dir_saida = format!("{}/{}/{}",&path_padrao,mes_ano,"saída");
    
    
                abrir_xml("0",dir_xml,dir_entrada,dir_saida,chave.to_string());

            };    
            

            
        }


    }else if verificador_nfe_saida{


        let line_atual:Vec<&str> = line_sped.split("|").collect();


        let tipo_doc = line_atual[5];

        if tipo_doc == "55" || tipo_doc == "65"{
            
            let chave = pegar_chave(&line_sped);

            if chave.len() != 0 {
                let mes_ano =  &format!("{}_{}",&line_atual[11][2..4],&line_atual[11][4..8])[..];

                let ano = &line_atual[11][4..];



                //vrificar e criar diretorio raiz

                

            

                let  path_padrao = format!("{}/{}","./organizado",ano);
                
                criar_diretorios(mes_ano, ano,dir_sped);
                
                let  dir_entrada = format!("{}/{}/{}",&path_padrao,mes_ano,"entrada");
                let  dir_saida = format!("{}/{}/{}",&path_padrao,mes_ano,"saída");
    
    
                abrir_xml("1",dir_xml,dir_entrada,dir_saida,chave.to_string());

            };    
            
        }
        
    }else if verificador_cfe{
        
                let line_atual:Vec<&str> = line_sped.split("|").collect();
        

                let chave = pegar_chave(&line_sped);

                if chave.len() != 0 {
        
        
                    let mes_ano =  &format!("{}_{}",&line_atual[5][2..4],&line_atual[5][4..8])[..];
        
                    let ano = &line_atual[5][4..];
    

    
                    let  path_padrao = format!("{}/{}","./organizado",ano);
                    
                    criar_diretorios(mes_ano, ano,dir_sped);
                    
                    let  dir_entrada = format!("{}/{}/{}",&path_padrao,mes_ano,"entrada");
                    let  dir_saida = format!("{}/{}/{}",&path_padrao,mes_ano,"saída");
        
        
                    abrir_xml("1",dir_xml,dir_entrada,dir_saida,chave.to_string());
    
                };    
                

    }
    
    

}

fn criar_diretorios(mes_ano:&str,ano:&str, dir_sped: &String){

    let path_padrao = "./organizado";

    verificar_criar_dir(path_padrao);

    let path_padrao = &format!("{}/{}",path_padrao,ano)[..];
    
    verificar_criar_dir(path_padrao);
    
    let path_padrao = &format!("{}/{}",path_padrao,mes_ano)[..];
    
    verificar_criar_dir(path_padrao);

    mover_sped(dir_sped, path_padrao.to_string());
    
    let path_padrao = &format!("{}/{}",path_padrao,"entrada")[..];
    
    verificar_criar_dir(path_padrao);
    
    let path_padrao = &format!("{}/{}/{}/{}","./organizado",ano,mes_ano,"saída")[..];
    
    verificar_criar_dir(path_padrao);
    


}

fn abrir_xml(tipo:&str,dir_xml:String,mut dir_entrada:String,mut dir_saida : String, chave:String){

    let file_xml = match  fs::File::open(&dir_xml){
        
        Ok(file) => file,
        Err(err)=> panic!("{}",err)
    };
    
    
    let mut bufferread = BufReader::new(file_xml);

    let mut texto = String::new();

    bufferread.read_to_string(&mut texto).unwrap();


    
    dir_entrada = format!("{}/{}.xml",dir_entrada,chave);
    dir_saida = format!("{}/{}.xml",dir_saida,chave);


    if tipo == "0"{
        
        println!("{}", dir_entrada);
        let mut file_xml = fs::File::create(dir_entrada).unwrap();
        
        file_xml.write_all(texto.as_bytes()).unwrap();
        
    }else if tipo == "1"{
        
        println!("{}", dir_saida);
        
        let mut file_xml = fs::File::create(dir_saida).unwrap();

        file_xml.write_all(texto.as_bytes()).unwrap();

    }

    


}
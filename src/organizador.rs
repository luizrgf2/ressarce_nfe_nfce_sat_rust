mod diretorio;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::env;


fn ler_arquivo(path:String) -> Vec<String>{


    let mut linhas = Vec::new();//variavel amazenara as linhas do sped

    let file = fs::File::open(path).unwrap();

    let mut buferread = BufReader::new(file);

    let mut buffer = Vec::new();

    buferread.read(&mut buffer);

    let texto = String::from_utf8_lossy(&buffer);

    for linha in texto.lines(){

        linhas.push(linha.to_string());

    }

    linhas



}

pub fn let_sped(){

    let diretorios = diretorio::pegar_xmls_speds();

    let dirs_sped = diretorios.speds;
    let dirs_xmls = diretorios.xmls;


    let nome_sped = String::from("");

    for dir_sped in dirs_sped{



        let mut sped = Vec::new();
        //veirificando sistema operacional
        if env::consts::OS == "windows"{

            sped = dir_sped.split("\\").collect();
            
        }else{
            
            sped = dir_sped.split("/").collect();

        }

        let sped = sped[sped.len()-1]; //nome do sped

        //abrindo o sped.txt
        let linhas_sped = ler_arquivo(dir_sped);
        //analisando linha por linha do sped
        for linha in linhas_sped{

            //abrindo xmls
            for dir_xml in &dirs_xmls{

                if linha.contains(dir_xml){

                    organizar(dir_xml.to_string(), linha)


                }

            }

        }





    }

}

fn pegar_linhas_seped_importantes(){

    

}

fn organizar(dir_xml:String,line_sped:String){



}
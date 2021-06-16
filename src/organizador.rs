mod diretorio;
use std::fs;
use std::io::BufReader;
use std::io::Read;


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



        let sped =dir_sped.split("/"); 

        println!("{}",dir_sped);

    }

}
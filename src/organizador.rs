use walkdir::WalkDir;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;



fn pegar_dirs() -> Vec<String>{


    let dirs = WalkDir::new("./arquivos");
    let mut xml_dirs_text = String::from("");
    let mut sped_dirs = Vec::new();


    for dir in dirs{


        let dir_atual = dir.unwrap().clone(); //diretorio atual


        if dir_atual.path().is_dir(){ //verificando se o diretorios atual é um aquivo
            
           
           
            if xml_dirs_text.len() == 0 && dir_atual.path().display().to_string().contains(".xml"){ 
    
                xml_dirs_text = format!("{}", &dir_atual.path().display());
    
            }else if dir_atual.path().display().to_string().contains(".xml"){
    
                xml_dirs_text = format!("{}\n{}",xml_dirs_text,&dir_atual.path().display());
    
            }else{

                sped_dirs.push(format!("{}",dir_atual.clone().path().display().to_string()));

            }



        };
        


    }



    let mut file = File::create("./dirs.txt").unwrap();

    file.write_all(xml_dirs_text.as_bytes()).unwrap();

    sped_dirs
}


import hmc = require("hmc-win32");
import fs = require("fs");
import nsfw = require('pre-nsfw');
import path = require('path');
import shake = require("hmc-shake");
import log  = require('electron-log');

log.initialize({ preload: true });

const nameList = ["DeviceTube.rs","libNcPlus.rs","main_view.rs","main.rs","util.rs"];

log.info("::nameList:: "+ nameList.join(","));
 
nsfw(
    path.resolve("./"),
    function(events) {
    for (let index = 0; index < events.length; index++) {
        const wat = events[index];
        if(wat.action == nsfw.ActionType.MODIFIED&&wat.file){
            
            let filePath = path.resolve(wat.directory, wat.file); 
            // log.info("::watcher::"+filePath);

            if(shake.isset("build",3000)&&nameList.includes(wat.file)){
                
                log.info("::kill:: -> "+
                hmc.killProcessName('HM神秘口袋',"hmpocketSecrets.exe").map(function(value){
                    return value.pid;
                }).join(','));
                
                log.info("::cargo build::");
                hmc.openApp('cmd','/c"cargo build  --release"',undefined,true,true);
            
                // log.info("::cargo run::");
                // hmc.openApp('cmd','/c"cargo run"',undefined,true,true);
            
            }
            if(wat.file =="hmpocketSecrets.exe"){
                
                if(wat.directory .includes("target\\release")){
                    log.info("::build->OK:: "+filePath);    
                    log.info("::build->OPEN::");
                    if(!hmc.hasProcess('HM神秘口袋',"hmpocketSecrets.exe")&&shake.isset("OPEN",3000))
                    hmc.openApp(filePath,undefined,process.cwd(),false, true)
                }

                if(wat.directory .includes("target\\debug")){
                    log.info("::build->OK:: "+filePath);    
                }

               
            }
        }
    }

    },{"debounceMS":800})
    .then(function(watcher) {
     log.info("::start:: ");
      return watcher.start();
    })
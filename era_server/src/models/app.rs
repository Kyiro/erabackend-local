use crate::files::*;
use crate::models::athena::{SlotVariants, Variant};
use crate::models::cloudstorage::SystemEntry;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::{env, fs, path::Path};
use std::io::{
    BufReader,
    BufWriter,
    Error as IoError,
    ErrorKind,
    Result as IoResult
};
use std::io::prelude::*;

#[derive(Default)]
pub struct State {
    pub cosmetics: Vec<CItem>,
    pub game: Value,
    pub keychain: Vec<String>,
    pub status: Value,
    pub user_path: String
}

impl State {
    pub async fn new() -> Self {
        let user_path = env::var("LOCALAPPDATA").unwrap_or(
            env::var("USERPROFILE")
            .unwrap_or(env::var("APPDATA").unwrap())
        );
        
        let mut state = Self::default();
        
        state.user_path = user_path.clone();
        
        let offline = if let Ok(config) = state.get_config() {
            config.offline
        } else { true };
        
        if offline == true {
            log::warn!("Offline mode is ON");
        }
        
        Self {
            cosmetics: cosmetics(offline).await,
            game: fortnite_game(offline).await,
            keychain: keychain(offline).await,
            status: status(offline).await,
            user_path
        }
    }
    
    pub fn get_config(&self) -> Result<Config> {
        let path = Path::new(&self.user_path).join("ProjectEra");
        fs::create_dir_all(path.to_str().unwrap())?;
        
        if let Ok(mut file) = fs::File::open(path.join("config.json")) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            Ok(serde_json::from_str(&contents)?)
        } else if let Ok(mut file) = fs::File::create(path.join("config.json")) {
            let config = Config::new();
            
            file.write_all(&serde_json::to_vec(&config)?)?;
            
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
    
    pub fn save_config(&self, config: &Config) -> Result<()> {
        let path = Path::new(&self.user_path).join("ProjectEra");
        let mut file = fs::File::create(path.join("config.json"))?;
        
        file.write_all(&serde_json::to_vec_pretty(config)?)?;
        
        Ok(())
    }
    
    pub fn list_user_cloudstorage(&self, season: usize) -> IoResult<Vec<SystemEntry>> {
        let dir = Path::new(&self.user_path).join("ProjectEra");
        let cloudstorage = dir.join("cloudstorage").join("user").join(season.to_string());
        let mut entries = Vec::new();
        
        fs::create_dir_all(cloudstorage.clone())?;
        
        for file in fs::read_dir(cloudstorage)? {
            let path = file?.path();
            
            if !path.is_file() {
                continue;
            }
            
            entries.push(SystemEntry::from_file(path)?);
        }
        
        Ok(entries)
    }
    
    pub fn get_user_file(&self, season: usize, file: String) -> IoResult<Vec<u8>> {
        let dir = Path::new(&self.user_path).join("ProjectEra");
        let cloudstorage = dir.join("cloudstorage").join("user").join(season.to_string());
        let file = cloudstorage.join(file);
        
        fs::create_dir_all(cloudstorage)?;
        
        if !file.is_file() {
            return Err(IoError::new(
                ErrorKind::NotFound,
                "The requested CloudStorage file was not found"
            ));
        }
        
        let mut buf_reader = BufReader::new(fs::File::open(file)?);
        let mut data = Vec::new();
        
        buf_reader.read_to_end(&mut data)?;
        
        Ok(data)
    }
    
    pub fn save_user_file(&self, season: usize, file: String, data: Vec<u8>) -> IoResult<()> {
        let dir = Path::new(&self.user_path).join("ProjectEra");
        let cloudstorage = dir.join("cloudstorage").join("user").join(season.to_string());
        
        fs::create_dir_all(cloudstorage.clone())?;
        
        let file = fs::File::create(cloudstorage.join(file))?;
        let mut buf_writer = BufWriter::new(file);
        
        buf_writer.write_all(&data)?;
        
        Ok(())
    }
    
    pub fn get_item(&self, item: CItem) -> Option<CItem> {
        for i in &self.cosmetics {
            if item.id == i.id && item.item_type == i.item_type {
                return Some(i.clone())
            }
        }
        
        None
    }
}

fn get_false() -> bool { false }

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "get_false")]
    pub offline: bool,
    #[serde(default = "String::new")]
    pub username: String,
    #[serde(default = "Locker::new")]
    pub locker: Locker,
    #[serde(default = "Vec::new")]
    pub favourites: Vec<String>,
    #[serde(default = "HashMap::new")]
    pub variants: HashMap<String, Vec<Variant>>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub id: String,
    pub variants: Vec<CVariant>,
}

impl CItem {
    pub fn new(id: String, item_type: String, variants: Vec<CVariant>) -> Self {
        Self {
            id,
            item_type,
            variants
        }
    }
    
    pub fn from_string(string: String) -> Self {
        let parts = string.split(":").collect::<Vec<&str>>();
        
        Self {
            id: parts.get(1).unwrap_or(&"").to_string(),
            item_type: parts.get(0).unwrap_or(&"").to_string(),
            variants: Vec::new()
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CVariant {
    pub channel: String,
    pub options: Vec<String>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Locker {
    pub name: String,
    
    pub character: LoadoutSlots<1>,
    pub backpack: LoadoutSlots<1>,
    pub pickaxe: LoadoutSlots<1>,
    pub glider: LoadoutSlots<1>,
    pub contrail: LoadoutSlots<1>,
    pub music_pack: LoadoutSlots<1>,
    pub loading: LoadoutSlots<1>,
    pub dance: LoadoutSlots<6>,
    pub item_wrap: LoadoutSlots<7>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LoadoutSlots<const LENGTH: usize> {
    #[serde(with = "serde_arrays")]
    pub items: [String; LENGTH],
    #[serde(with = "serde_arrays")]
    pub variants: [Option<SlotVariants>; LENGTH]
}

impl<const LENGTH: usize> Default for LoadoutSlots<LENGTH> {
    fn default() -> Self {
        fn to_arr<T, const LEN: usize>(vector: Vec<T>) -> [T; LEN] {
            vector.try_into().unwrap_or_else(|_| panic!("Couldn't convert to an array"))
        }
        
        let mut empty_items = Vec::new();
        let mut empty_variants = Vec::new();
        
        for _ in 0..LENGTH {
            empty_items.push(String::default());
            empty_variants.push(None);
        }
        
        Self {
            items: to_arr(empty_items),
            variants: to_arr(empty_variants)
        }
    }
}

impl<const LENGTH: usize> LoadoutSlots<LENGTH> {
    pub fn new(items: [String; LENGTH]) -> Self {
        let mut loadout_slots = Self::default();
        
        loadout_slots.items = items;
        
        loadout_slots
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            offline: false,
            username: String::from("Project Era"),
            locker: Locker::new(),
            favourites: Vec::new(),
            variants: HashMap::new()
        }
    }
}

impl Locker {
    pub fn new() -> Self {
        let mut locker = Self::default();
        
        locker.name = String::from("Project Era");
        locker.character = LoadoutSlots::new(["AthenaCharacter:cid_005_athena_commando_m_default".to_string()]);
        locker.pickaxe = LoadoutSlots::new(["AthenaPickaxe:defaultpickaxe".to_string()]);
        locker.glider = LoadoutSlots::new(["AthenaPickaxe:defaultglider".to_string()]);
        
        locker
    }
    
    pub fn get(&self, slot: String) -> Option<(&[String], &[Option<SlotVariants>])> {
        Some(match slot.to_lowercase().as_str() {
            "character" => (&self.character.items, &self.character.variants),
            "backpack" => (&self.backpack.items, &self.backpack.variants),
            "pickaxe" => (&self.pickaxe.items, &self.pickaxe.variants),
            "glider" => (&self.glider.items, &self.glider.variants),
            "skydivecontrail" => (&self.contrail.items, &self.contrail.variants),
            "musicpack" => (&self.music_pack.items, &self.music_pack.variants),
            "loadingscreen" => (&self.loading.items, &self.loading.variants),
            "dance" => (&self.dance.items, &self.dance.variants),
            "itemwrap" => (&self.item_wrap.items, &self.item_wrap.variants),
            _ => None?
        })
    }
    
    pub fn get_mut(&mut self, slot: String) -> Option<(&mut [String], &mut [Option<SlotVariants>])> {
        Some(match slot.to_lowercase().as_str() {
            "character" => (&mut self.character.items, &mut self.character.variants),
            "backpack" => (&mut self.backpack.items, &mut self.backpack.variants),
            "pickaxe" => (&mut self.pickaxe.items, &mut self.pickaxe.variants),
            "glider" => (&mut self.glider.items, &mut self.glider.variants),
            "skydivecontrail" => (&mut self.contrail.items, &mut self.contrail.variants),
            "musicpack" => (&mut self.music_pack.items, &mut self.music_pack.variants),
            "loadingscreen" => (&mut self.loading.items, &mut self.loading.variants),
            "dance" => (&mut self.dance.items, &mut self.dance.variants),
            "itemwrap" => (&mut self.item_wrap.items, &mut self.item_wrap.variants),
            _ => None?
        })
    }
}
use std::{fs, io, path};

pub struct HeadFile {
    file: io::BufReader<fs::File>,
}
impl HeadFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("HEAD");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
pub struct ConfigFile {
    file: io::BufReader<fs::File>,
}
impl ConfigFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("config");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
pub struct DescriptionFile {
    file: io::BufReader<fs::File>,
}
impl DescriptionFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("description");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}

pub struct HooksFile {
    file: io::BufReader<fs::File>,
}
impl HooksFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("hooks");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
pub struct InfoFile {
    file: io::BufReader<fs::File>,
}
impl InfoFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("info");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
pub struct ObjectsFile {
    file: io::BufReader<fs::File>,
}
impl ObjectsFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("objects");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
pub struct RefsFile {
    file: io::BufReader<fs::File>,
}
impl RefsFile {
    pub fn new(path: &str) -> io::Result<Self> {
        let path = path::Path::new(path);
        let path = path.join("refs");
        let file = fs::File::create(path)?;
        Ok(Self {
            file: io::BufReader::new(file),
        })
    }
}
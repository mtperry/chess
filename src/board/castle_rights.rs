#[allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both        
}

impl CastleRights {
    pub fn has_kingside(self) -> bool {
        if self == CastleRights::KingSide || self == CastleRights::Both {
            true
        } else {
            false
        }
    }

    pub fn has_queenside(self) -> bool {
        if self == CastleRights::QueenSide || self == CastleRights::Both {
            true
        } else {
            false
        }
    }

    pub fn remove_kingside(&mut self) {
        *self = match *self {
            CastleRights::KingSide => CastleRights::None,
            CastleRights::Both     => CastleRights::QueenSide,
            _ => *self,
        }
    }

    pub fn remove_queenside(&mut self) {
        *self = match *self {
            CastleRights::QueenSide => CastleRights::None,
            CastleRights::Both      => CastleRights::KingSide,
            _ => *self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn has_kingside() {
        let rights = CastleRights::Both;
        assert!(rights.has_kingside());

        let rights = CastleRights::KingSide;
        assert!(rights.has_kingside());

        let rights = CastleRights::QueenSide;
        assert!(!rights.has_kingside());

        let rights = CastleRights::None;
        assert!(!rights.has_kingside());
    }

    #[test]
    fn has_queenside() {
        let rights = CastleRights::Both;
        assert!(rights.has_queenside());

        let rights = CastleRights::KingSide;
        assert!(!rights.has_queenside());

        let rights = CastleRights::QueenSide;
        assert!(rights.has_queenside());
        
        let rights = CastleRights::None;
        assert!(!rights.has_queenside());
    }
    
    #[test]
    fn remove_kingside() {
        let mut rights = CastleRights::Both;
        rights.remove_kingside();
        assert_eq!(rights, CastleRights::QueenSide);
        
        rights = CastleRights::KingSide;
        rights.remove_kingside();
        assert_eq!(rights, CastleRights::None);

        rights = CastleRights::QueenSide;
        rights.remove_kingside();
        assert_eq!(rights, CastleRights::QueenSide);

        rights = CastleRights::None;
        rights.remove_kingside();
        assert_eq!(rights, CastleRights::None);
    }

    #[test]
    fn remove_queenside() {
        let mut rights = CastleRights::Both;
        rights.remove_queenside();
        assert_eq!(rights, CastleRights::KingSide);
        
        rights = CastleRights::QueenSide;
        rights.remove_queenside();
        assert_eq!(rights, CastleRights::None);

        rights = CastleRights::KingSide;
        rights.remove_queenside();
        assert_eq!(rights, CastleRights::KingSide);

        rights = CastleRights::None;
        rights.remove_queenside();
        assert_eq!(rights, CastleRights::None);
    }
}



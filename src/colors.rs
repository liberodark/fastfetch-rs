use crossterm::style::Color;
use std::collections::HashMap;

pub fn get_logo_colors(os_name: &str) -> HashMap<String, Color> {
    let mut colors = HashMap::new();

    macro_rules! insert_colors {
        ($color_array:expr) => {
            for (i, color) in $color_array.iter().enumerate() {
                colors.insert(format!("${}", i + 1), *color);
            }
        };
    }

    macro_rules! insert_ansi256_colors {
        ($color_numbers:expr) => {
            for (i, num) in $color_numbers.iter().enumerate() {
                colors.insert(format!("${}", i + 1), Color::AnsiValue(*num));
            }
        };
    }

    match os_name.to_lowercase().as_str() {
        "adelie" | "adélie" => {
            insert_colors!(&[Color::Blue, Color::White, Color::Cyan]);
        }
        "aeros" => {
            insert_colors!(&[Color::Cyan, Color::Cyan]);
        }
        "afterglow" => {
            insert_colors!(&[Color::Magenta, Color::Red, Color::Yellow, Color::Blue]);
        }
        "aix" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "almalinux" => {
            insert_colors!(&[
                Color::Red,
                Color::Yellow,
                Color::Blue,
                Color::Green,
                Color::Cyan
            ]);
        }
        "alpine" | "alpinelinux" | "alpine-linux" => {
            insert_colors!(&[Color::Blue]);
        }
        "alpine_small" | "alpine-linux-small" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "alpine2_small" | "alpine-linux2-small" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "alter" => {
            insert_colors!(&[Color::Cyan]);
        }
        "amazon" => {
            insert_colors!(&[Color::Yellow, Color::DarkYellow]);
        }
        "anarchy" => {
            insert_colors!(&[Color::Blue]);
        }
        "android" => {
            insert_colors!(&[Color::Green]);
        }
        "antergos" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "antiX" | "antix" => {
            insert_colors!(&[Color::Red, Color::Cyan, Color::White]);
        }
        "aosc" | "aosc_os" | "aosc-os" => {
            insert_colors!(&[Color::Blue, Color::Black]);
        }
        "aperture" => {
            insert_colors!(&[Color::White]);
        }
        "apricity" => {
            insert_colors!(&[Color::White, Color::Blue]);
        }
        "archcraft" => {
            insert_colors!(&[
                Color::Cyan,
                Color::Green,
                Color::Red,
                Color::Yellow,
                Color::Blue,
                Color::Magenta
            ]);
        }
        "arch" | "archlinux" | "arch-linux" => {
            insert_colors!(&[Color::Cyan, Color::Cyan]);
        }
        "archmerge" => {
            insert_colors!(&[Color::Cyan, Color::Cyan]);
        }
        "arcolinux" => {
            insert_colors!(&[Color::Blue]);
        }
        "arkane" | "arkanelinux" => {
            insert_colors!(&[Color::Green, Color::Red, Color::Yellow]);
        }
        "armbian" => {
            insert_colors!(&[Color::Red]);
        }
        "artix" | "artixlinux" => {
            insert_colors!(&[Color::Cyan]);
        }
        "arya" => {
            insert_colors!(&[
                Color::Yellow,
                Color::Cyan,
                Color::Red,
                Color::Red,
                Color::White,
                Color::Black,
                Color::Cyan
            ]);
        }
        "aster" => {
            insert_colors!(&[Color::Cyan]);
        }
        "asteroidos" => {
            insert_ansi256_colors!(&[160, 208, 202, 214]);
        }
        "astOS" | "astos" => {
            insert_colors!(&[Color::White]);
        }
        "astra" | "astra linux" | "astra_linux" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "athena" | "athenaos" | "athenaOS" => {
            insert_colors!(&[Color::White, Color::Yellow]);
        }
        "azos" => {
            insert_colors!(&[Color::Cyan, Color::Red]);
        }
        "backarcher" => {
            insert_colors!(&[Color::Red]);
        }
        "bedrock" => {
            insert_colors!(&[Color::DarkGrey, Color::White]);
        }
        "biglinux" => {
            insert_colors!(&[Color::Cyan, Color::Yellow, Color::Blue]);
        }
        "bitrig" => {
            insert_colors!(&[Color::Green]);
        }
        "blackarch" => {
            insert_colors!(&[Color::Red, Color::Red, Color::Black]);
        }
        "blackmesa" | "black-mesa" => {
            insert_colors!(&[Color::Black]);
        }
        "blackpanther" => {
            insert_colors!(&[Color::Red, Color::Yellow, Color::Blue]);
        }
        "blag" => {
            insert_colors!(&[Color::Magenta]);
        }
        "blankon" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "bluelight" => {
            insert_colors!(&[Color::White, Color::Blue]);
        }
        "bodhi" => {
            insert_colors!(&[Color::White, Color::Yellow, Color::Green]);
        }
        "bonsai" => {
            insert_colors!(&[Color::Cyan, Color::Green, Color::Yellow]);
        }
        "bsd" => {
            insert_colors!(&[Color::Red, Color::White, Color::Blue, Color::Red]);
        }
        "bunsenlabs" => {
            insert_colors!(&[Color::DarkGrey, Color::Grey, Color::Yellow]);
        }
        "cachyos" => {
            insert_colors!(&[Color::Cyan, Color::Green]);
        }
        "calculate" | "calculatelinux" => {
            insert_colors!(&[Color::DarkYellow, Color::White]);
        }
        "calinixos" => {
            insert_colors!(&[Color::Green, Color::Yellow]);
        }
        "carbs" => {
            insert_colors!(&[Color::Blue]);
        }
        "cbpp" | "crunchbangplusplus" => {
            insert_colors!(&[Color::White]);
        }
        "centos" => {
            insert_colors!(&[
                Color::Yellow,
                Color::Green,
                Color::Blue,
                Color::Magenta,
                Color::White
            ]);
        }
        "cereus" | "cereus linux" => {
            insert_ansi256_colors!(&[173, 108, 71, 151, 72]);
        }
        "chakra" => {
            insert_colors!(&[Color::Blue]);
        }
        "chaletos" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "chapeau" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "chimera linux" => {
            insert_colors!(&[Color::Red, Color::Magenta, Color::Blue, Color::Red]);
        }
        "chonkysealos" => {
            insert_colors!(&[Color::White]);
        }
        "clear-linux-os" | "clear_linux" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "cleanjaro" => {
            insert_colors!(&[Color::White]);
        }
        "clearos" => {
            insert_colors!(&[Color::Green, Color::Yellow, Color::Red]);
        }
        "clover" => {
            insert_colors!(&[Color::Green, Color::Cyan]);
        }
        "cobalt" => {
            insert_colors!(&[Color::Blue]);
        }
        "condres" => {
            insert_colors!(&[Color::Green, Color::Yellow, Color::Red]);
        }
        "container_linux" | "coreos" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "crystal" | "crystal linux" => {
            insert_colors!(&[Color::Magenta, Color::Black]);
        }
        "dahlia" => {
            insert_colors!(&[Color::Red, Color::Green]);
        }
        "debian" => {
            insert_colors!(&[Color::Red, Color::DarkRed]);
        }
        "deepin" => {
            insert_colors!(&[Color::Green, Color::Cyan]);
        }
        "desaos" => {
            insert_colors!(&[Color::Green]);
        }
        "devuan" => {
            insert_colors!(&[Color::DarkGrey]);
        }
        "dietpi" => {
            insert_colors!(&[Color::Green, Color::Black]);
        }
        "dragonfly" | "dragonflybsd" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "drauger" => {
            insert_colors!(&[Color::Red]);
        }
        "droidian" => {
            insert_colors!(&[Color::Green]);
        }
        "elementary" | "elementaryos" => {
            insert_colors!(&[Color::Blue, Color::Black]);
        }
        "elive" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "endeavour" | "endeavouros" => {
            insert_colors!(&[Color::Magenta, Color::Red, Color::Blue]);
        }
        "endless" => {
            insert_colors!(&[Color::DarkYellow, Color::Red]);
        }
        "enso" => {
            insert_colors!(&[Color::Black]);
        }
        "eshanizedos" => {
            insert_colors!(&[Color::Red]);
        }
        "eurolinux" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "exherbo" => {
            insert_colors!(&[Color::Blue, Color::Red]);
        }
        "fedora" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "femboyos" => {
            insert_colors!(&[Color::Magenta, Color::White, Color::Magenta]);
        }
        "feren" => {
            insert_colors!(&[Color::Blue]);
        }
        "finnix" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "freebsd" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "freemint" => {
            insert_colors!(&[Color::White]);
        }
        "frugalware" => {
            insert_colors!(&[Color::Blue, Color::DarkGrey]);
        }
        "funtoo" => {
            insert_colors!(&[Color::Magenta, Color::White]);
        }
        "furretos" => {
            insert_colors!(&[Color::Blue, Color::Cyan, Color::White]);
        }
        "galliumos" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "garuda" => {
            insert_colors!(&[Color::Red]);
        }
        "gentoo" => {
            insert_colors!(&[Color::Magenta, Color::White]);
        }
        "gnome" => {
            insert_colors!(&[Color::Blue]);
        }
        "gnewsense" => {
            insert_colors!(&[Color::Blue]);
        }
        "gnu" => {
            insert_colors!(&[Color::White, Color::Yellow]);
        }
        "gobolinux" => {
            insert_colors!(&[Color::Yellow, Color::Green, Color::Blue]);
        }
        "grapheneos" => {
            insert_colors!(&[Color::Blue, Color::Green, Color::Yellow, Color::Cyan]);
        }
        "grombyang" => {
            insert_colors!(&[Color::Blue, Color::Cyan, Color::White]);
        }
        "guix" => {
            insert_colors!(&[Color::Yellow, Color::White]);
        }
        "haiku" => {
            insert_colors!(&[Color::Green, Color::Black]);
        }
        "hamara" => {
            insert_colors!(&[Color::Green]);
        }
        "hardenedbsd" => {
            insert_colors!(&[Color::DarkYellow, Color::White]);
        }
        "hash" => {
            insert_colors!(&[Color::Green, Color::Black]);
        }
        "huayra" => {
            insert_colors!(&[Color::Blue]);
        }
        "hydroOS" | "hydroos" => {
            insert_colors!(&[Color::Blue, Color::Green]);
        }
        "hyperbola" => {
            insert_colors!(&[Color::DarkGrey]);
        }
        "iglunix" => {
            insert_colors!(&[Color::White]);
        }
        "instantos" => {
            insert_colors!(&[Color::Blue]);
        }
        "interix" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "irix" => {
            insert_colors!(&[Color::Blue]);
        }
        "januslinux" => {
            insert_colors!(&[Color::Blue, Color::Magenta]);
        }
        "kaisen" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "kali" => {
            insert_colors!(&[Color::Blue, Color::Black]);
        }
        "kaos" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "kde" | "kde-neon" | "kde_neon" => {
            insert_colors!(&[Color::Green, Color::Blue]);
        }
        "kibojoe" => {
            insert_colors!(&[Color::Green, Color::White, Color::Blue]);
        }
        "kiss" => {
            insert_colors!(&[Color::Magenta]);
        }
        "kogaion" => {
            insert_colors!(&[Color::Blue]);
        }
        "korora" => {
            insert_colors!(&[Color::Blue]);
        }
        "kdeneon" => {
            insert_colors!(&[Color::Blue]);
        }
        "kubuntu" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "laxeros" => {
            insert_colors!(&[Color::White, Color::Black]);
        }
        "lede" => {
            insert_colors!(&[Color::Blue]);
        }
        "libreelec" => {
            insert_colors!(&[Color::Green, Color::Yellow]);
        }
        "lingmo" | "lingmo os" | "lingmo_os" => {
            insert_colors!(&[Color::Blue, Color::Green]);
        }
        "linux" => {
            insert_colors!(&[Color::Black, Color::White]);
        }
        "linuxlite" | "linux_lite" | "linux-lite" => {
            insert_colors!(&[Color::Yellow, Color::White]);
        }
        "linuxmint" | "mint" | "linux mint" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "lmde" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "lubuntu" => {
            insert_colors!(&[Color::Blue, Color::Yellow]);
        }
        "mageia" => {
            insert_colors!(&[Color::Cyan, Color::Magenta]);
        }
        "magpieos" => {
            insert_colors!(&[Color::Green, Color::Yellow]);
        }
        "mainsailos" => {
            insert_colors!(&[Color::Blue, Color::Green]);
        }
        "manjaro" => {
            insert_colors!(&[Color::Green]);
        }
        "massos" => {
            insert_colors!(&[Color::White, Color::Red]);
        }
        "maui" => {
            insert_colors!(&[Color::Cyan, Color::White]);
        }
        "meowix" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "mer" => {
            insert_colors!(&[Color::Blue]);
        }
        "minios" => {
            insert_colors!(&[Color::Yellow, Color::DarkGrey]);
        }
        "mint_old" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "miraclelinux" => {
            insert_colors!(&[Color::Blue]);
        }
        "moeOS" | "moeos" => {
            insert_colors!(&[Color::Cyan, Color::White, Color::Red]);
        }
        "monjaro" => {
            insert_colors!(&[Color::Yellow]);
        }
        "morphos" => {
            insert_colors!(&[Color::Blue, Color::Grey]);
        }
        "mxlinux" | "mx" | "mx-linux" => {
            insert_colors!(&[Color::White, Color::Black]);
        }
        "namib" => {
            insert_colors!(&[Color::DarkYellow]);
        }
        "neon" => {
            insert_colors!(&[Color::Blue]);
        }
        "neptune" => {
            insert_colors!(&[Color::White]);
        }
        "netbsd" => {
            insert_colors!(&[Color::DarkRed, Color::White]);
        }
        "netrunner" => {
            insert_colors!(&[Color::Blue]);
        }
        "nitrux" => {
            insert_colors!(&[Color::Blue]);
        }
        "nixos" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "nurunner" => {
            insert_colors!(&[Color::Blue]);
        }
        "nutyx" => {
            insert_colors!(&[Color::Blue, Color::Red]);
        }
        "nyaarch" => {
            insert_colors!(&[Color::Cyan, Color::White, Color::Black]);
        }
        "obarun" => {
            insert_colors!(&[Color::Cyan]);
        }
        "obrevenge" => {
            insert_colors!(&[Color::DarkGrey, Color::Red]);
        }
        "omarine" => {
            insert_colors!(&[Color::Blue]);
        }
        "omnios" => {
            insert_colors!(&[Color::White, Color::Yellow]);
        }
        "openbsd" => {
            insert_colors!(&[Color::Yellow, Color::White, Color::Cyan, Color::Red]);
        }
        "openeuler" => {
            insert_colors!(&[Color::Blue]);
        }
        "openindiana" => {
            insert_colors!(&[Color::Blue]);
        }
        "opensuse" | "suse" | "open_suse" | "open-suse" => {
            insert_colors!(&[Color::Green]);
        }
        "opensuse_microos" | "opensuse-microos" => {
            insert_colors!(&[Color::Green]);
        }
        "opensuse_leap" | "opensuse leap" => {
            insert_colors!(&[Color::White]);
        }
        "opensuse_leap_old" | "opensuse leap_old" => {
            insert_colors!(&[Color::White]);
        }
        "opensuse-tumbleweed" | "opensuse_tumbleweed" => {
            insert_colors!(&[Color::White]);
        }
        "opensuse-tumbleweed-old" | "opensuse_tumbleweed-old" => {
            insert_colors!(&[Color::White]);
        }
        "opensuse_slowroll" | "opensuse-slowroll" | "opensuse-tumbleweed-slowroll" => {
            insert_colors!(&[Color::White]);
        }
        "openmandriva" | "open-mandriva" | "open_mandriva" | "openmandriva lx" => {
            insert_colors!(&[Color::Blue]);
        }
        "openwrt" => {
            insert_colors!(&[Color::Blue]);
        }
        "opnsense" => {
            insert_colors!(&[Color::DarkYellow, Color::DarkGrey]);
        }
        "oracle" | "oraclelinux" | "oracle linux" | "oracle linux server" => {
            insert_colors!(&[Color::Red]);
        }
        "orchid" => {
            insert_colors!(&[Color::White, Color::Magenta, Color::Magenta]);
        }
        "os_elbrus" | "os elbrus" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "osmc" | "open source media center" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "osx" => {
            insert_colors!(&[
                Color::Green,
                Color::Yellow,
                Color::Red,
                Color::Magenta,
                Color::Blue
            ]);
        }
        "pacbsd" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "panwah" => {
            insert_colors!(&[Color::White, Color::Red]);
        }
        "parabola" => {
            insert_colors!(&[Color::Magenta]);
        }
        "pardus" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "parrot" => {
            insert_colors!(&[Color::Green, Color::Cyan]);
        }
        "parsix" => {
            insert_colors!(&[Color::Yellow, Color::Red, Color::DarkGrey]);
        }
        "pear" | "pearos" => {
            insert_colors!(&[Color::Green, Color::Yellow, Color::DarkYellow]);
        }
        "pengwin" => {
            insert_colors!(&[Color::Magenta, Color::DarkGrey]);
        }
        "pentoo" => {
            insert_colors!(&[Color::Magenta, Color::White]);
        }
        "peppermint" => {
            insert_colors!(&[Color::Red, Color::Black]);
        }
        "peropesis" | "peropesis linux" => {
            insert_colors!(&[Color::White]);
        }
        "phyos" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "pika" | "pikaos" => {
            insert_colors!(&[Color::Yellow]);
        }
        "pisi" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "pnm" | "pnm linux" => {
            insert_colors!(&[Color::Blue, Color::Red, Color::White]);
            colors.insert("$4".to_string(), Color::AnsiValue(202));
        }
        "popos" | "pop_os" | "pop!_os" | "pop" => {
            insert_colors!(&[Color::Cyan, Color::White]);
        }
        "porteus" => {
            insert_colors!(&[Color::Cyan, Color::White]);
        }
        "postmarketos" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "proxmox" | "pve" => {
            colors.insert("$1".to_string(), Color::White);
            colors.insert("$2".to_string(), Color::AnsiValue(202));
        }
        "pureos" => {
            insert_colors!(&[Color::Green]);
        }
        "puffos" => {
            insert_colors!(&[Color::Yellow, Color::White]);
        }
        "puppy" => {
            insert_colors!(&[Color::Cyan]);
        }
        "q4os" => {
            insert_colors!(&[Color::Blue, Color::Red]);
        }
        "qubes" => {
            insert_colors!(&[Color::Blue]);
        }
        "qubyt" => {
            insert_colors!(&[Color::Cyan]);
        }
        "quibian" => {
            insert_colors!(&[Color::Yellow]);
        }
        "radix" => {
            insert_colors!(&[Color::Green, Color::Red]);
        }
        "raspbian" => {
            insert_colors!(&[Color::Red, Color::Green]);
        }
        "ravynos" => {
            colors.insert("$1".to_string(), Color::AnsiValue(15));
            colors.insert("$2".to_string(), Color::White);
        }
        "reborn" | "rebornos" | "reborn os" | "reborn-os" => {
            insert_colors!(&[Color::Black, Color::Blue, Color::Cyan]);
        }
        "red_star" | "redstar" | "redstar-os" | "redstaros" => {
            insert_colors!(&[Color::Red]);
        }
        "redcore" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "redhat" | "redhat enterprise linux" | "rhel" => {
            insert_colors!(&[Color::Red]);
        }
        "redos" | "red os" | "red-os" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "refracta" | "refracted devuan" | "refracted-devuan" => {
            insert_colors!(&[Color::Black, Color::DarkGrey]);
        }
        "regata" => {
            insert_colors!(&[Color::Red, Color::Green, Color::Blue]);
        }
        "regolith" => {
            insert_colors!(&[Color::Red, Color::DarkGrey]);
        }
        "rhaymos" => {
            insert_colors!(&[
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta
            ]);
        }
        "rocky" | "rockylinux" => {
            insert_colors!(&[Color::Green]);
        }
        "rosa" => {
            insert_colors!(&[Color::Blue]);
        }
        "sabayon" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "sabotage" => {
            insert_colors!(&[Color::White]);
        }
        "sailfishos" => {
            insert_colors!(&[Color::Blue]);
        }
        "salentos" => {
            insert_colors!(&[Color::Green, Color::Red, Color::White]);
        }
        "salientos" => {
            insert_colors!(&[Color::Yellow, Color::White, Color::Blue, Color::Yellow]);
        }
        "salix" => {
            insert_colors!(&[Color::Green]);
        }
        "samberos" => {
            insert_colors!(&[Color::Yellow, Color::Green, Color::Red, Color::White]);
        }
        "sasanqua" => {
            insert_colors!(&[Color::Magenta, Color::Blue]);
        }
        "scientific" => {
            insert_colors!(&[Color::Blue, Color::Red]);
        }
        "semc" => {
            insert_colors!(&[Color::Green, Color::Red]);
        }
        "septor" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "serene" => {
            insert_colors!(&[Color::Cyan]);
        }
        "sharklinux" => {
            insert_colors!(&[Color::Blue]);
        }
        "shastraos" => {
            insert_colors!(&[Color::DarkYellow, Color::Cyan, Color::Red]);
        }
        "siduction" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "skinux" => {
            insert_colors!(&[Color::Blue, Color::Black, Color::White]);
        }
        "skiffos" => {
            insert_colors!(&[Color::Blue, Color::Black]);
        }
        "slackware" => {
            insert_colors!(&[Color::Blue, Color::DarkBlue]);
        }
        "slax" => {
            insert_colors!(&[Color::Cyan]);
        }
        "slitaz" => {
            insert_colors!(&[Color::Yellow, Color::DarkGrey]);
        }
        "smartos" => {
            insert_colors!(&[Color::Cyan]);
        }
        "soda" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "solaris" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "solus" => {
            insert_colors!(&[Color::Blue, Color::DarkGrey, Color::Cyan, Color::DarkGrey]);
        }
        "source_mage" | "source-mage" => {
            insert_colors!(&[Color::Red]);
        }
        "sparky" => {
            insert_colors!(&[Color::Red]);
        }
        "springdale" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "star" => {
            insert_colors!(&[Color::White, Color::Blue]);
        }
        "steamos" => {
            insert_colors!(&[Color::Cyan, Color::Black]);
        }
        "stock" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "sulin" => {
            insert_colors!(&[Color::Green]);
        }
        "sunos" => {
            insert_colors!(&[Color::White, Color::Red]);
        }
        "swagarch" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "t2" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "tails" => {
            insert_colors!(&[Color::Blue, Color::Green]);
        }
        "tatra" => {
            insert_colors!(&[Color::Blue]);
        }
        "tearch" => {
            insert_colors!(&[Color::White, Color::Blue, Color::Black]);
        }
        "terrame" => {
            insert_colors!(&[Color::Green, Color::Red]);
        }
        "tinycore" => {
            insert_colors!(&[Color::Yellow, Color::White]);
        }
        "tokaos" => {
            insert_colors!(&[Color::Blue, Color::Green, Color::Yellow]);
        }
        "torizoncore" => {
            insert_colors!(&[Color::Blue, Color::Green, Color::Yellow]);
        }
        "trisquel" => {
            insert_colors!(&[Color::Blue, Color::Cyan]);
        }
        "tuxedoos" => {
            insert_colors!(&[Color::DarkYellow, Color::Black]);
        }
        "ubuntu" => {
            insert_colors!(&[Color::DarkRed, Color::Red]);
        }
        "ubuntu-budgie" | "ubuntu_budgie" => {
            insert_colors!(&[Color::Blue, Color::White]);
        }
        "ubuntu-cinnamon" | "ubuntu_cinnamon" => {
            insert_colors!(&[Color::DarkRed]);
        }
        "ubuntu-gnome" | "ubuntu_gnome" => {
            insert_colors!(&[Color::DarkRed, Color::Magenta, Color::White]);
        }
        "ubuntu-kylin" | "ubuntu_kylin" => {
            insert_colors!(&[Color::DarkRed, Color::White]);
        }
        "ubuntu-mate" | "ubuntu_mate" => {
            insert_colors!(&[Color::Green, Color::White]);
        }
        "ubuntu-studio" | "ubuntu_studio" => {
            insert_colors!(&[Color::Blue, Color::DarkRed]);
        }
        "ubuntu-sway" | "ubuntu_sway" => {
            insert_colors!(&[Color::DarkRed, Color::Blue]);
        }
        "ubuntu-touch" | "ubuntu_touch" => {
            insert_colors!(&[Color::DarkRed, Color::White]);
        }
        "ubuntu-unity" | "ubuntu_unity" => {
            insert_colors!(&[Color::Magenta]);
        }
        "ultramarine" => {
            insert_colors!(&[Color::Blue]);
        }
        "univalent" => {
            insert_colors!(&[Color::Cyan, Color::Yellow]);
        }
        "univention" => {
            insert_colors!(&[Color::Red, Color::White]);
        }
        "uruk" => {
            insert_colors!(&[Color::Black, Color::Red, Color::White]);
        }
        "uwuntu" => {
            insert_ansi256_colors!(&[225, 206, 52]);
        }
        "valhalla" | "valhallaos" | "valhalla-linux" => {
            insert_colors!(&[Color::White]);
        }
        "vanilla" | "vanilla-os" | "vanilla-linux" => {
            insert_colors!(&[Color::Yellow]);
        }
        "vanilla2" | "vanilla-os2" | "vanilla-linux2" => {
            insert_colors!(&[Color::Yellow]);
        }
        "vanilla-small" | "vanilla-os-small" | "vanilla-linux-small" => {
            insert_colors!(&[Color::Yellow, Color::DarkYellow]);
        }
        "venom" => {
            insert_colors!(&[Color::DarkGrey, Color::Blue]);
        }
        "vnux" => {
            colors.insert("$1".to_string(), Color::AnsiValue(11));
            colors.insert("$2".to_string(), Color::AnsiValue(8));
            colors.insert("$3".to_string(), Color::AnsiValue(15));
            colors.insert("$4".to_string(), Color::Red);
            colors.insert("$5".to_string(), Color::White);
        }
        "void" | "void-linux" => {
            insert_colors!(&[Color::Green, Color::Black]);
        }
        "vzlinux" => {
            insert_colors!(&[Color::Red, Color::White, Color::Yellow]);
        }
        "wiilinuxngx" | "wii-linux-ngx" => {
            insert_colors!(&[Color::Cyan, Color::White]);
        }
        "windows server 2025" => {
            insert_colors!(&[Color::Blue, Color::Blue, Color::Blue, Color::Blue]);
        }
        "windows 11" | "windows server 2022" => {
            insert_colors!(&[Color::Blue, Color::Blue, Color::Blue, Color::Blue]);
        }
        "windows 11_small" | "windows 11-small" => {
            insert_colors!(&[Color::Blue, Color::Blue, Color::Blue, Color::Blue]);
        }
        "windows 8"
        | "windows 8.1"
        | "windows 10"
        | "windows server 2012"
        | "windows server 2012 r2"
        | "windows server 2016"
        | "windows server 2019" => {
            insert_colors!(&[Color::Cyan, Color::Cyan, Color::Cyan, Color::Cyan]);
        }
        "windows" | "windows 7" | "windows server 2008" | "windows server 2008 r2" => {
            insert_colors!(&[Color::Red, Color::Green, Color::Blue, Color::Yellow]);
        }
        "windows 95" | "windows 9x" => {
            insert_colors!(&[
                Color::Cyan,
                Color::Blue,
                Color::Yellow,
                Color::Green,
                Color::Red,
                Color::Black
            ]);
        }
        "xenia" => {
            insert_colors!(&[Color::Yellow, Color::Green, Color::Red]);
        }
        "xcp-ng" | "xenenterprise" => {
            insert_colors!(&[
                Color::Red,
                Color::Red,
                Color::Black,
                Color::Black,
                Color::Blue,
                Color::Yellow
            ]);
        }
        "xeroarch" => {
            insert_ansi256_colors!(&[50, 14, 50, 93, 16, 15]);
        }
        "xray_os" => {
            insert_ansi256_colors!(&[15, 14, 16, 24]);
        }
        "xferience" => {
            insert_colors!(&[Color::Cyan, Color::Cyan]);
        }
        "xubuntu" => {
            colors.insert("$1".to_string(), Color::AnsiValue(25));
            colors.insert("$2".to_string(), Color::White);
        }
        "yiffos" => {
            insert_ansi256_colors!(&[93, 92]);
        }
        "zorin" | "zorinos" | "zorin-linux" | "zorinos-linux" => {
            insert_colors!(&[Color::Blue]);
        }
        "z/os" | "zos" => {
            insert_colors!(&[Color::Blue]);
        }
        _ => {
            insert_colors!(&[Color::White, Color::Grey]);
        }
    }

    colors
}

/// Convert ANSI color names to crossterm Color enum
#[allow(dead_code)]
pub fn ansi_to_crossterm_color(ansi_name: &str) -> Option<Color> {
    match ansi_name {
        "FF_COLOR_FG_BLACK" => Some(Color::Black),
        "FF_COLOR_FG_RED" => Some(Color::DarkRed),
        "FF_COLOR_FG_GREEN" => Some(Color::DarkGreen),
        "FF_COLOR_FG_YELLOW" => Some(Color::DarkYellow),
        "FF_COLOR_FG_BLUE" => Some(Color::DarkBlue),
        "FF_COLOR_FG_MAGENTA" => Some(Color::DarkMagenta),
        "FF_COLOR_FG_CYAN" => Some(Color::DarkCyan),
        "FF_COLOR_FG_WHITE" => Some(Color::Grey),
        "FF_COLOR_FG_DEFAULT" => Some(Color::Reset),

        "FF_COLOR_FG_LIGHT_BLACK" => Some(Color::DarkGrey),
        "FF_COLOR_FG_LIGHT_RED" => Some(Color::Red),
        "FF_COLOR_FG_LIGHT_GREEN" => Some(Color::Green),
        "FF_COLOR_FG_LIGHT_YELLOW" => Some(Color::Yellow),
        "FF_COLOR_FG_LIGHT_BLUE" => Some(Color::Blue),
        "FF_COLOR_FG_LIGHT_MAGENTA" => Some(Color::Magenta),
        "FF_COLOR_FG_LIGHT_CYAN" => Some(Color::Cyan),
        "FF_COLOR_FG_LIGHT_WHITE" => Some(Color::White),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_logos() {
        let test_cases = vec![
            ("nixos", 2),
            ("arch", 2),
            ("ubuntu", 2),
            ("debian", 2),
            ("fedora", 2),
            ("manjaro", 1),
            ("gentoo", 2),
            ("void", 2),
            ("alpine", 1),
            ("centos", 5),
            ("windows", 4),
            ("windows 11", 4),
            ("uwuntu", 3),
            ("xeroarch", 6),
        ];

        for (distro, expected_colors) in test_cases {
            let colors = get_logo_colors(distro);
            assert_eq!(
                colors.len(),
                expected_colors,
                "Expected {} colors for {}, got {}",
                expected_colors,
                distro,
                colors.len()
            );
        }
    }

    #[test]
    fn test_color_keys() {
        let colors = get_logo_colors("arch");
        assert!(colors.contains_key("$1"));
        assert!(colors.contains_key("$2"));
        assert!(!colors.contains_key("$0"));
        assert!(!colors.contains_key("$10"));
    }

    #[test]
    fn test_case_insensitive() {
        let colors_lower = get_logo_colors("nixos");
        let colors_upper = get_logo_colors("NIXOS");
        let colors_mixed = get_logo_colors("NixOS");

        assert_eq!(colors_lower.len(), colors_upper.len());
        assert_eq!(colors_lower.len(), colors_mixed.len());
    }

    #[test]
    fn test_aliases() {
        let arch_colors = get_logo_colors("arch");
        let archlinux_colors = get_logo_colors("archlinux");
        let arch_linux_colors = get_logo_colors("arch-linux");

        assert_eq!(arch_colors.len(), archlinux_colors.len());
        assert_eq!(arch_colors.len(), arch_linux_colors.len());
    }

    #[test]
    fn test_default_fallback() {
        let colors = get_logo_colors("unknown_distro_12345");
        assert_eq!(colors.len(), 2);
        assert_eq!(colors.get("$1"), Some(&Color::White));
        assert_eq!(colors.get("$2"), Some(&Color::Grey));
    }

    #[test]
    fn test_ansi256_colors() {
        let uwuntu_colors = get_logo_colors("uwuntu");
        assert_eq!(uwuntu_colors.get("$1"), Some(&Color::AnsiValue(225)));
        assert_eq!(uwuntu_colors.get("$2"), Some(&Color::AnsiValue(206)));
        assert_eq!(uwuntu_colors.get("$3"), Some(&Color::AnsiValue(52)));

        let xeroarch_colors = get_logo_colors("xeroarch");
        assert_eq!(xeroarch_colors.len(), 6);
        assert_eq!(xeroarch_colors.get("$1"), Some(&Color::AnsiValue(50)));
    }
}

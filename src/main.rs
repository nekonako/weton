use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct T {
    nama: String,
    keterangan: String,
}

fn main() {
    let tabel_hari: HashMap<String, u8> = HashMap::from([
        (String::from("minggu"), 5),
        (String::from("senin"), 4),
        (String::from("selasa"), 3),
        (String::from("rabu"), 7),
        (String::from("kamis"), 8),
        (String::from("jumat"), 6),
        (String::from("sabtu"), 9),
    ]);

    let tabel_pasaran: HashMap<String, u8> = HashMap::from([
        (String::from("wage"), 4),
        (String::from("kliwon"), 8),
        (String::from("legi"), 5),
        (String::from("pahing"), 9),
        (String::from("pon"), 7),
    ]);

    let tabel_hoki: HashMap<u8, T> = HashMap::from([
        (
            1,
            T {
                nama: String::from("Wasesa Segara"),
                keterangan: String::from("banyak rezeki dan segala perbuatanya baik"),
            },
        ),
        (
            2,
            T {
                nama: String::from("Tunggak Semi"),
                keterangan: String::from("rezekinya selalu lancar"),
            },
        ),
        (
            3,
            T {
                nama: String::from("Satria Wibawa"),
                keterangan: String::from("selalu bisa menemukan keenakan atau kemudahan"),
            },
        ),
        (
            4,
            T {
                nama: String::from("Sumur Sinaba"),
                keterangan: String::from("dapat menjadi tempat pengungsian"),
            },
        ),
        (
            5,
            T {
                nama: String::from("Satria Wirang"),
                keterangan: String::from("dapat menjadi petani"),
            },
        ),
        (
            6,
            T {
                nama: String::from("Bumi Kepetak"),
                keterangan: String::from("selalu medapatkan rintangan"),
            },
        ),
        (
            7,
            T {
                nama: String::from("Lebu Ketiaup Angin"),
                keterangan: String::from(
                    "selalu berada dalam kekurangan dan sering berpindah tempat",
                ),
            },
        ),
    ]);

    let tabel_jodoh: HashMap<Vec<u8>, T> = HashMap::from([
        (
            vec![1, 9, 10, 18, 19, 27, 28, 36],
            T {
                nama: String::from("Pegat"),
                keterangan: String::from("Masalah yang sering ditemui oleh pasangan PEGAT ini di kemudian hari mulai dari masalah ekonomi, kekuasaan, perselingkuhan yang bisa menyebabkan pasangan tersebut bercerai atau pegatan."),
            },
        ),
        (
            vec![2, 11, 20, 29],
            T {
                nama: String::from("Ratu"),
                keterangan: String::from("Bisa dibilang pasangan tersebut memang sudah jodohnya. Dihargai dan disegani oleh tetangga dan lingkungan sekitar. Saking harmonisnya, bahkan banyak orang yang iri akan keharmonisannya dalam membina rumah tangga."),
            },
        ),
        (
            vec![3, 12, 21, 39],
            T {
                nama: String::from("Jodoh"),
                keterangan: String::from("Pasangan tersebut memang beneran cocok dan berjodoh. Pasangan ini bisa saling menerima segala kelebihan dan kekurangan masing-masing. Rumah tangga pasangan JODOH ini bisa rukun sampai tua."),
            },
        ),
        (
            vec![4, 13, 22, 31],
            T {
                nama: String::from("Topo"),
                keterangan: String::from("Dalam membina rumah tangga, pasangan TOPO akan sering mengalami kesusahan di awal musim karena masih saling memahami tapi akan bahagia pada akhirnya. Masalah yang dihadapi bisa saja soal ekonomi dan lainnya. Nah, saat sudah memiliki anak dan cukup lama berumah tangga, akhirnya akan hidup sukses dan bahagia."),
            },
        ),
        (
            vec![5, 14, 23, 32],
            T {
                nama: String::from("Tinari"),
                keterangan: String::from("Pasangan TINARI akan menemukan kebahagiaan. Dalam mencari rezeki diberikan kemudahan dan nggak sampai hidup kekurangan. Selain itu, hidupnya juga sering mendapat keberuntungan."),
            },
        ),
        (
            vec![6, 15, 24, 33],
            T {
                nama: String::from("Padu"),
                keterangan: String::from("Dalam berumah tangga, pasangan PADU akan sering mengalami pertengkaran. Tapi Bela, meskipun sering bertengkar, nggak sampai cerai. Masalah pertengkaran tersebut bahkan bisa dipicu dari hal-hal yang sifatnya cukup sepele."),
            },
        ),
        (
            vec![7, 16, 25, 34],
            T {
                nama: String::from("Sujanan"),
                keterangan: String::from("Dalam berumah tangga, pasangan SUJANAN akan sering mengalami pertengkaran dan masalah perselingkuhan. Bisa itu dari pihak laki-laki maupun perempuan yang memulai perselingkuhan tersebut."),
            },
        ),
        (
            vec![8, 17, 26, 35],
            T {
                nama: String::from("Pesthi"),
                keterangan: String::from("Umumnya, pasangan dengan hasil hitangan weton Jawa pesthi akan rukun, tenteram, dan damai. Meskipun nanti ada masalah yang menerpa, pasangan ini dinilai akan tetap harmonis dan bahagia."),
            },
        ),
    ]);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\n format perintah tidak valid");
        println!("$ weton hari(senin/selasa/rabu/kamis/juamt/sabtu/minggu) pasaran(waget/kliwon/legi/pahing/pon)");
        println!("$ weton senin kliwon \n");
        println!("atau untuk jodoh");
        println!("$ weton hari pasaran jodoh_hari jodoh_pasaran");
        println!("$ weton selasa kliwon senin pahing \n");
        return
    }

    let dino = &args[1];
    let pasaran = &args[2];

    let neptumu = neptu(tabel_hari.get(dino), tabel_pasaran.get(pasaran));
    let hoki = tabel_hoki.get(&neptumu);

    println!("weton nasib : {}", &hoki.unwrap().nama,);
    println!("keterangan : {}", &hoki.unwrap().keterangan);

    if args.len() == 5 {
        let jodoh_dino = &args[3];
        let jodoh_pasaran = &args[4];
        let neptu_jodoh = neptu(tabel_hari.get(jodoh_dino), tabel_pasaran.get(jodoh_pasaran));
        let total_neptu = neptumu + neptu_jodoh;
        for (key, val) in &tabel_jodoh {
            if key.contains(&total_neptu) {
                println!("weton pasangan : {}", val.nama);
                println!("keterangan : {}", val.keterangan)
            };
        }
    }
}

fn neptu(hari: Option<&u8>, pasaran: Option<&u8>) -> u8 {
    if hari.is_some() && pasaran.is_some() {
        return (hari.unwrap() + pasaran.unwrap()) % 7;
    }
    return 0;
}

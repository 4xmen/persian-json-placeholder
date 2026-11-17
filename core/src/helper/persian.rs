use rand::RngExt;
use rand::prelude::IndexedRandom;

const WORDS: [&str; 99] = [
    "سیب",
    "سیگار",
    "دریا",
    "سنگین",
    "مخفی",
    "سرافکنده",
    "خنده‌دار",
    "پرسپولیس",
    "لواشک",
    "ترش",
    "مشکی",
    "گرم",
    "فرهنگی",
    "خودمانی",
    "خوش‌بو",
    "باران",
    "پنجره",
    "خیابان",
    "آینه",
    "چراغ",
    "نارنجی",
    "بنفش",
    "ابری",
    "شکلات",
    "فندق",
    "گردو",
    "پسته",
    "هندوانه",
    "توت‌فرنگی",
    "گیلاس",
    "پاییز",
    "تابستان",
    "زمستان",
    "بهاری",
    "ابریشم",
    "مخمل",
    "چوبی",
    "طلایی",
    "نقره‌ای",
    "شیشه‌ای",
    "براق",
    "کج‌ومعوج",
    "عجیب",
    "آرام",
    "شلوغ",
    "تنبل",
    "باهوش",
    "کنجکاو",
    "خواب‌آلود",
    "خندان",
    "خجالتی",
    "رنگارنگ",
    "خوش‌مزه",
    "ترش‌وشیرین",
    "خنک",
    "داغ",
    "نرم",
    "زبر",
    "سبک",
    "پشمی",
    "پنبه‌ای",
    "کاغذی",
    "موزیک",
    "رادیو",
    "سینما",
    "دوچرخه",
    "چمدان",
    "چتر",
    "کفش",
    "جوراب",
    "کاپشن",
    "کافه",
    "بستنی",
    "قهوه",
    "نسکافه",
    "ساندویچ",
    "پیتزا",
    "ماکارونی",
    "زعفران",
    "دارچین",
    "هل",
    "نعناع",
    "ریحان",
    "بادبادک",
    "حباب",
    "مرکزی",
    "کهکشان",
    "سیاره",
    "ابر",
    "نسیم",
    "موج",
    "ساحل",
    "صدف",
    "ماهی",
    "لاک‌پشت",
    "گربه",
    "روباه",
    "پروانه",
    "زرافه",
];
pub fn persian_lorem(prefix: bool, word_count: usize) -> String {
    let mut rng = rand::rng();
    let mut result = Vec::with_capacity(word_count * 12);

    if prefix {
        result.push("لورم ".to_string());
        result.push("متن ".to_string());
        result.push("بی‌معنی ".to_string());
    }

    let mut paragraph_size = rng.random_range(30..=70);
    let mut paragraph_words = 0;

    for i in 0..word_count {
        if paragraph_words >= paragraph_size {
            result.push("\n\n".to_string());
            paragraph_size = rng.random_range(30..=70);
            paragraph_words = 0;
        }

        let word = WORDS.choose(&mut rng).unwrap();
        result.push(word.to_string());

        paragraph_words += 1;

        if i < word_count - 1 {
            result.push(" ".to_string());
        }
    }

    result.concat()
}
pub fn persian_lorem_html(prefix: bool, word_count: usize, new_line: &str) -> String {
    let mut rng = rand::rng();
    let mut result = Vec::with_capacity(word_count * 12);

    if prefix {
        result.push("لورم ".to_string());
        result.push("متن ".to_string());
        result.push("بی‌معنی ".to_string());
    }

    let mut paragraph_size = rng.random_range(30..=70);
    let mut paragraph_words = 0;

    for i in 0..word_count {
        if paragraph_words >= paragraph_size {
            result.push(new_line.to_string());
            paragraph_size = rng.random_range(30..=70);
            paragraph_words = 0;
        }

        let word = WORDS.choose(&mut rng).unwrap();
        result.push(word.to_string());

        paragraph_words += 1;

        if i < word_count - 1 {
            result.push(" ".to_string());
        }
    }

    result.concat()
}

pub fn persian_slug() -> (String, String) {
    let words = [
        ("زرد", "zard"),
        ("مشکی", "meshki"),
        ("طلایی", "talayi"),
        ("سیب", "sib"),
        ("ویژه", "vijeh"),
        ("زیبا", "ziba"),
        ("قرمز", "ghermez"),
        ("آبی", "abi"),
        ("سبز", "sabz"),
        ("سفید", "sefid"),
        ("نارنجی", "narenji"),
        ("بنفش", "banafsh"),
        ("صورتی", "soorati"),
        ("نقره‌ای", "noghreyi"),
        ("قهوه‌ای", "ghahvei"),
        ("خاکستری", "khakestari"),
        ("آسمان", "aseman"),
        ("دریا", "darya"),
        ("باران", "baran"),
        ("برف", "barf"),
        ("خورشید", "khorshid"),
        ("ماه", "mah"),
        ("ستاره", "setareh"),
        ("ابر", "abr"),
        ("نسیم", "nasim"),
        ("گل", "gol"),
        ("لاله", "laleh"),
        ("یاس", "yas"),
        ("رز", "roz"),
        ("سنبل", "sonbol"),
        ("بهار", "bahar"),
        ("تابستان", "tabestan"),
        ("پاییز", "payiz"),
        ("زمستان", "zemestan"),
        ("جنگل", "jangal"),
        ("کوه", "kooh"),
        ("دشت", "dasht"),
        ("رود", "rood"),
        ("دریاچه", "daryacheh"),
        ("جزیره", "jazireh"),
        ("شیرین", "shirin"),
        ("خوشحال", "khoshhal"),
        ("آرام", "aram"),
        ("سریع", "sarie"),
        ("هوشمند", "hooshmand"),
        ("قشنگ", "ghashang"),
        ("دوست", "doost"),
        ("امید", "omid"),
        ("رویا", "roya"),
        ("لبخند", "labkhand"),
        ("کتاب", "ketab"),
        ("قلم", "ghalam"),
        ("دفتر", "daftar"),
        ("چراغ", "cheragh"),
        ("پنجره", "panjereh"),
        ("خانه", "khaneh"),
        ("باغ", "bagh"),
        ("میدان", "meydan"),
        ("راه", "rah"),
        ("سفر", "safar"),
        ("سیب‌زمینی", "sibzamini"),
        ("انار", "anar"),
        ("انگور", "angoor"),
        ("هلو", "holoo"),
        ("گیلاس", "gilas"),
        ("خرما", "khorma"),
        ("لیمو", "limoo"),
        ("هندوانه", "hendevaneh"),
        ("توت", "toot"),
        ("گردو", "gerdoo"),
        ("تازه", "tazeh"),
        ("خوشمزه", "khoshmazeh"),
        ("درخشان", "derakhshan"),
        ("قوی", "ghavi"),
        ("نرم", "narm"),
        ("گرم", "garm"),
        ("خنک", "khanak"),
        ("روشن", "roshan"),
        ("تاریک", "tarik"),
        ("پاک", "pak"),
        ("آزاد", "azad"),
        ("شاد", "shad"),
        ("خندان", "khandan"),
        ("مهربان", "mehraban"),
        ("شجاع", "shoja"),
        ("طلوع", "toloo"),
        ("غروب", "ghorob"),
        ("رنگین‌کمان", "ranginkaman"),
        ("آذرخش", "azarkhesh"),
        ("زمرد", "zamorod"),
        ("یاقوت", "yaghoot"),
        ("مروارید", "morvarid"),
        ("عقیق", "aghigh"),
        ("کاج", "kaj"),
        ("سرو", "sarv"),
        ("چنار", "chenar"),
        ("پرستو", "parastoo"),
        ("پروانه", "parvaneh"),
        ("شبنم", "shabnam"),
    ];

    let mut rng = rand::rng();
    let (fa, finglish) = words.choose(&mut rng).unwrap();
    let number: u16 = rng.random_range(1111..=9999);
    (fa.to_string(), format!("{finglish}{number}"))
}

use std::path::Path;

use model::Model;
use errors::LoadError;
use utils::read_file;


pub struct Classifier {
    models: Vec<(String, Model)>
}


impl Classifier {
    pub fn new() -> Self {
        Classifier { models: Vec::new() }
    }

    pub fn from_files<P: AsRef<Path>, I: IntoIterator<Item=P>>(paths: I) -> Result<Self, LoadError> {
        let mut classifier = Classifier::new();
        for path in paths {
            let path = path.as_ref();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let mut buf: Vec<u8> = Vec::new();
            try!(read_file(path, &mut buf));
            let model = try!(Model::deserialize(buf));
            classifier.add_model(model, name);
        }
        Ok(classifier)
    }

    pub fn train(&mut self, text: &str, name: &str) {
        let model = Model::build_from_text(text);
        self.add_model(model, name);
    }

    pub fn add_model(&mut self, model: Model, name: &str) {
        self.models.push((name.to_owned(), model));
    }

    pub fn classify(&self, text: &str) -> &String {
        let model = Model::build_from_text(text);
        let &(ref name, _) = self.models
            .iter()
            .min_by_key(|&&(_, ref other_model)| model.compare(other_model))
            .unwrap();
        name
    }
}


#[cfg(test)]
mod tests {
    extern crate glob;

    use super::*;
    use self::glob::glob;

    #[test]
    fn classification() {
        let en_training_text = "Estimates of the number of languages in the world vary between 5,000 and 7,000. However, any precise estimate depends on a partly arbitrary distinction between languages and dialects. Natural languages are spoken or signed, but any language can be encoded into secondary media using auditory, visual, or tactile stimuli – for example, in graphic writing, braille, or whistling. This is because human language is modality-independent. Depending on philosophical perspectives regarding the definition of language and meaning, when used as a general concept, 'language' may refer to the cognitive ability to learn and use systems of complex communication, or to describe the set of rules that makes up these systems, or the set of utterances that can be produced from those rules. All languages rely on the process of semiosis to relate signs to particular meanings. Oral and sign languages contain a phonological system that governs how symbols are used to form sequences known as words or morphemes, and a syntactic system that governs how words and morphemes are combined to form phrases and utterances.";
        let ru_training_text = "Фрида Кало родилась 6 июля 1907 года в Койоакане, пригороде Мехико (позднее она поменяла год рождения на 1910 — год Мексиканской революции). Её отцом был фотограф Гильермо Кало, немец еврейского происхождения. Мать Фриды, Матильда Кальдерон, была мексиканкой с индейскими корнями. Фрида Кало была третьим ребёнком в семье. В 6 лет она перенесла полиомиелит, после болезни на всю жизнь осталась хромота, а её правая нога стала тоньше левой (что Кало всю жизнь скрывала под длинными юбками). Столь ранний опыт борьбы за право полноценной жизни закалил характер Фриды. Фрида занималась боксом и другими видами спорта. В 15 лет она поступила в «Препараторию» (Национальную подготовительную школу), одну из лучших школ Мексики, с целью изучать медицину. Из 2000 учащихся в этой школе было всего 35 девушек. Фрида сразу же заработала авторитет, создав с восемью другими учащимися закрытую группу «Качучас». Её поведение часто называли эпатажным. В Препаратории произошла её первая встреча с будущим мужем, известным мексиканским художником Диего Риверой, с 1921 по 1923 работавшим в Подготовительной школе над росписью «Созидание». В возрасте восемнадцати лет 17 сентября 1925 года Фрида попала в тяжёлую аварию. Автобус, на котором она ехала, столкнулся с трамваем. Фрида получила серьёзные травмы: тройной перелом позвоночника (в поясничной области), перелом ключицы, сломанные рёбра, тройной перелом таза, одиннадцать переломов костей правой ноги, раздробленную и вывихнутую правую стопу, вывихнутое плечо. Кроме того, её живот и матка были проколоты металлическим перилом, что серьёзно повредило её репродуктивную функцию. Она год была прикована к кровати, а проблемы со здоровьем остались на всю жизнь. Впоследствии Фриде пришлось перенести несколько десятков операций, месяцами не выходя из больниц. Она, несмотря на горячее желание, так и не смогла стать матерью. Именно после трагедии она впервые попросила у отца кисти и краски. Для Фриды сделали специальный подрамник, позволявший писать лёжа. Под балдахином кровати прикрепили большое зеркало, чтобы она могла видеть себя. Первой картиной был автопортрет, что навсегда определило основное направление творчества: «Я пишу себя, потому что много времени провожу в одиночестве и потому что являюсь той темой, которую знаю лучше всего». В 1928 году вступила в Мексиканскую коммунистическую партию. В 1929 году Фрида Кало стала женой Диего Риверы. Ему было 43 года, ей — 22. Сближало двух художников не только искусство, но и общие политические убеждения — коммунистические. Их бурная совместная жизнь стала легендой. Спустя много лет Фрида говорила: «В моей жизни было две аварии: одна — когда автобус врезался в трамвай, другая — это Диего». В 1930-х годах Фрида какое-то время жила в США, где работал муж. Это вынужденное долгое пребывание за границей, в развитой индустриальной стране, заставило её острее чувствовать национальные различия. С тех пор Фрида с особенной любовью относилась к народной мексиканской культуре, коллекционировала старинные произведения прикладного искусства, даже в повседневной жизни носила национальные костюмы. Поездка в Париж в 1939, где Фрида стала сенсацией тематической выставки мексиканского искусства (одна из её картин была даже приобретена Лувром), ещё сильнее развила патриотическое чувство. В 1937 году в доме Диего и Фриды ненадолго нашёл убежище советский революционный деятель Лев Троцкий; у них с Фридой завязался роман. Считается, что уехать от них его вынудило слишком явное увлечение темпераментной мексиканкой. В 1940-е годы картины Фриды появляются на нескольких заметных выставках. В то же время обостряются её проблемы со здоровьем. Лекарства и наркотики, призванные уменьшить физические страдания, меняют её душевное состояние, что ярко отражается в Дневнике, ставшем культовым среди её поклонников. В 1953 году состоялась её первая персональная выставка на родине. К тому времени Фрида уже не могла встать с постели, и на открытие выставки её принесли на больничной койке.. Вскоре из-за начавшейся гангрены ей ампутировали правую ногу ниже колена. Фрида Кало умерла 13 июля 1954 года от воспаления лёгких. Незадолго до смерти она оставила в своём дневнике последнюю запись: «Надеюсь, что уход будет удачным, и я больше не вернусь». Некоторые друзья Фриды Кало предполагали, она умерла от передозировки, и её смерть могла быть неслучайной. Тем не менее, доказательств этой версии не существует, вскрытие тела не проводилось. Прощание с Фридой Кало проходило во Дворце изящных искусств. На церемонии помимо Диего Риверы присутствовал президент Мексики Ласаро Карденас и многие деятели искусств. С 1955 года «Голубой дом» Фриды Кало стал музеем её памяти.";

        let mut classifier = Classifier::new();
        classifier.train(en_training_text, "en");
        classifier.train(ru_training_text, "ru");

        assert_eq!(
            classifier.classify("This is a sample of text in English language."),
            "en"
        );
        assert_eq!(
            classifier.classify("А это кажется пример текста на русском языке."),
            "ru"
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn from_files_iterators() {
        // This test just makes sure that from_files accepts various common kinds of input args
        let array = ["/dev/nonexistent", "/and/another"];
        let vector = vec!["/one/path.jpg", "/two/path.jpg"];
        let glob_iterator = glob("/dev/*").unwrap().filter_map(Result::ok);
        Classifier::from_files(&array);
        Classifier::from_files(&vector);
        Classifier::from_files(glob_iterator);
        assert!(true);
    }
}

pub struct Strings;

impl Strings {
    // App & Nav
    pub const APP_TITLE: &'static str = "Amategeko y'Umuhanda";
    pub const NAV_HOME: &'static str = "Ahabanza";
    pub const NAV_QUIZ: &'static str = "Ikizamini";
    pub const NAV_QUESTIONS: &'static str = "Ibibazo";
    pub const NAV_STATS: &'static str = "Imibare";
    pub const NAV_SETTINGS: &'static str = "Igenamiterere";

    // Home Screen
    pub const HOME_WELCOME: &'static str = "Witegure ikizamini cy'amategeko y'umuhanda mu Rwanda";
    pub const HOME_CHOOSE_MODE: &'static str = "Hitamo uburyo bwo kwitoza:";
    pub const HOME_RESUME_TITLE: &'static str = "Ufite ikizamini utarasoza";
    pub const HOME_RESUME_BTN: &'static str = "Komeza ikizamini";
    pub const HOME_DISCARD_BTN: &'static str = "Reka";
    pub const HOME_WEAK_TITLE: &'static str = "Ibibazo nakosheje kenshi";
    pub const HOME_WEAK_DESC: &'static str =
        "Itoze ibibazo ugiramo amakosa cyane kugira ngo ubyumve neza";
    pub const HOME_WEAK_BTN: &'static str = "Tangira kwitoza ibyo nakosheje";

    // Modes
    pub const MODE_EASY_TITLE: &'static str = "Byoroshye";
    pub const MODE_EASY_DESC: &'static str =
        "Uburyo bwo kwiga: igisubizo cyihuse, nta gihe kigabanyuka";
    pub const MODE_MEDIUM_TITLE: &'static str = "Hagati";
    pub const MODE_MEDIUM_DESC: &'static str =
        "Imyitozo y'ikizamini nyacyo: iminota 20, ushobora gusimbuka";
    pub const MODE_HARD_TITLE: &'static str = "Bikomeye";
    pub const MODE_HARD_DESC: &'static str =
        "Uburyo bukaze: iminota 12, nta gusubira inyuma cyangwa gusimbuka";

    // Quiz Common
    pub const QUIZ_QUESTION_PROGRESS: &'static str = "Ikibazo {current} kuri {total}";
    pub const QUIZ_PREVIOUS: &'static str = "Ibibanza";
    pub const QUIZ_NEXT: &'static str = "Ibikurikira";
    pub const QUIZ_SKIP: &'static str = "Simbuka";
    pub const QUIZ_CONFIRM_ANSWER: &'static str = "Emeza igisubizo";
    pub const QUIZ_FINISH: &'static str = "Soza ikizamini";
    pub const QUIZ_FLAG: &'static str = "Shyiraho ikimenyetso";
    pub const QUIZ_UNFLAG: &'static str = "Kura ikimenyetso";
    pub const QUIZ_STAR: &'static str = "Bika ikibazo";
    pub const QUIZ_UNSTAR: &'static str = "Kura mu bibitswe";
    pub const QUIZ_NO_IMAGE: &'static str = "Ishusho y'icyapa iza hano";
    pub const QUIZ_SHORTCUT_HINT: &'static str = "A-D cyangwa 1-4 uhitamo; Enter ugakomeza";

    // Quiz Feedback (Easy mode)
    pub const QUIZ_EASY_HINT: &'static str = "Kanda igisubizo kugira ngo ubone niba ari cyo";
    pub const QUIZ_EASY_CORRECT: &'static str = "Ni byo! Igisubizo cy'ukuri ni {option}.";
    pub const QUIZ_EASY_WRONG: &'static str = "Siko. Igisubizo cy'ukuri ni {option}.";

    // Hard mode errors
    pub const QUIZ_HARD_NO_SELECTION: &'static str = "Hitamo igisubizo mbere yo kwemeza";

    // Finish / Abandon Dialogs
    pub const DIALOG_FINISH_TITLE: &'static str = "Soza ikizamini?";
    pub const DIALOG_UNANSWERED_WARNING: &'static str =
        "Haracyari ibibazo {count} bitasubijwe: {list}";
    pub const DIALOG_ALL_ANSWERED: &'static str = "Ibibazo byose 20 byasubijwe neza.";
    pub const DIALOG_CONFIRM_FINISH: &'static str = "Emeza gusoza";
    pub const DIALOG_CANCEL: &'static str = "Komeza ikizamini";

    pub const DIALOG_ABANDON_TITLE: &'static str = "Guhagarika ikizamini?";
    pub const DIALOG_ABANDON_DESC_SAVABLE: &'static str =
        "Urashaka gusohoka? Ikizamini cyawe kirabikwa kugira ngo uzagikomeze.";
    pub const DIALOG_ABANDON_DESC_STRICT: &'static str =
        "Mu buryo bukaze, gusohoka bivuze ko ikizamini kizarangira burundu.";
    pub const DIALOG_ABANDON_CONFIRM: &'static str = "Sohoka";

    // Results Screen
    pub const RESULTS_TITLE: &'static str = "Ibisubizo by'ikizamini";
    pub const RESULTS_PASSED: &'static str = "Watsinze neza!";
    pub const RESULTS_FAILED: &'static str = "Ntiwatsinze, komeza witoze!";
    pub const RESULTS_SCORE: &'static str = "Wagize amanota {score} kuri {total}";
    pub const RESULTS_PASS_MARK: &'static str = "Amanota asabwa yo gutsinda ni {pass_mark}/{total}";
    pub const RESULTS_DURATION: &'static str = "Igihe wakoresheje: {duration}";
    pub const RESULTS_RETRY_WRONG: &'static str = "Subiramo ibyo wakosheje";
    pub const RESULTS_NEW_QUIZ: &'static str = "Tangira ikindi kizamini";
    pub const RESULTS_FILTER_ALL: &'static str = "Byose ({count})";
    pub const RESULTS_FILTER_CORRECT: &'static str = "Iby'ukuri ({count})";
    pub const RESULTS_FILTER_WRONG: &'static str = "Ibyakosheje ({count})";
    pub const RESULTS_YOUR_ANSWER: &'static str = "Igisubizo cyawe: {answer}";
    pub const RESULTS_CORRECT_ANSWER: &'static str = "Igisubizo cy'ukuri: {answer}";
    pub const RESULTS_UNANSWERED: &'static str = "Ntiwasubije";

    // Questions Screen (Question Bank)
    pub const QUESTIONS_SEARCH_PLACEHOLDER: &'static str =
        "Shakisha mu bibazo cyangwa andika numero...";
    pub const QUESTIONS_FILTER_ALL: &'static str = "Byose";
    pub const QUESTIONS_FILTER_IMAGE: &'static str = "Bifite amashusho";
    pub const QUESTIONS_FILTER_MISTAKES: &'static str = "Ibyo nakosheje";
    pub const QUESTIONS_FILTER_STARRED: &'static str = "Ibibitswe";
    pub const QUESTIONS_HIDE_ANSWERS: &'static str = "Hisha ibisubizo";
    pub const QUESTIONS_COUNT_INFO: &'static str = "Ibibazo {visible} kuri {total}";

    // Statistics Screen
    pub const STATS_TITLE: &'static str = "Imibare n'Iterambere";
    pub const STATS_TOTAL_ATTEMPTS: &'static str = "Ibizamini byakozwe";
    pub const STATS_AVERAGE_SCORE: &'static str = "Impuzandengo y'amanota";
    pub const STATS_HIGH_SCORE: &'static str = "Amanota yo hejuru";
    pub const STATS_PASS_RATE: &'static str = "Ijanisha ryo gutsinda";
    pub const STATS_RECENT_CHART_TITLE: &'static str = "Ibizamini 10 biheruka";
    pub const STATS_THRESHOLD_LABEL: &'static str = "Amanota asabwa";
    pub const STATS_TOP_MISSED_TITLE: &'static str = "Ibibazo bikunze gukoswa cyane";
    pub const STATS_MISSED_COUNT: &'static str = "Byakoswe inshuro {wrong} kuri {seen}";
    pub const STATS_QUESTIONS_SEEN: &'static str = "Ibibazo wamaze kubona: {seen} kuri {total}";
    pub const STATS_EMPTY: &'static str =
        "Nta kizamini urakora. Tangira ikizamini kugira ngo ubone imibare!";

    // Settings Screen
    pub const SETTINGS_TITLE: &'static str = "Igenamiterere";
    pub const SETTINGS_PASS_MARK: &'static str = "Amanota asabwa yo gutsinda (kuri 20)";
    pub const SETTINGS_MEDIUM_TIME: &'static str = "Igihe cy'ikizamini cya Hagati (iminota)";
    pub const SETTINGS_HARD_TIME: &'static str = "Igihe cy'ikizamini Gikomeye (iminota)";
    pub const SETTINGS_EASY_TIMER: &'static str = "Kwereka igihe muri Byoroshye";
    pub const SETTINGS_HARD_WEIGHT: &'static str =
        "Gushyira imbere ibyapa n'ibimenyetso mu Gikomeye";
    pub const SETTINGS_DESKTOP_SHORTCUTS: &'static str =
        "Gukoresha buto za clavier (A-D, Enter, F)";
    pub const SETTINGS_THEME: &'static str = "Insanganyamatsiko (Theme)";
    pub const SETTINGS_THEME_SYSTEM: &'static str = "Iya telefone / mudasobwa";
    pub const SETTINGS_THEME_LIGHT: &'static str = "Urumuri (Light)";
    pub const SETTINGS_THEME_DARK: &'static str = "Umwijima (Dark)";
    pub const SETTINGS_FONT_SIZE: &'static str = "Ingano y'inyandiko";
    pub const SETTINGS_FONT_PREVIEW: &'static str =
        "Urugero rw'inyandiko: Amategeko y'Umuhanda mu Rwanda";
    pub const SETTINGS_CLEAR_HISTORY: &'static str = "Gusiba amakuru n'ibizamini byakozwe";
    pub const SETTINGS_CLEAR_CONFIRM: &'static str = "Wizeye neza ko ushaka gusiba amakuru yose?";
    pub const SETTINGS_SAVE_BTN: &'static str = "Bika impinduka";
}

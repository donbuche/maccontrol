//! Interface languages. Texts are written in English in the code and looked
//! up here with `tl("…")`; a text missing from the table shows in English.
//! Flags are drawn in code (egui's fonts have no flag emoji). The language
//! button and its menu are the same as Quicksnap's.

use eframe::egui::{self, pos2, vec2, Align2, Color32, FontId, Painter, Rect, Sense, Stroke};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Lang {
    En,
    Es,
    Ca,
}

impl Lang {
    /// In the order of the translation table's columns (English first).
    const BY_INDEX: [Lang; 3] = [Lang::En, Lang::Es, Lang::Ca];

    pub const ALL: [Lang; 3] = [Lang::En, Lang::Es, Lang::Ca];

    /// The language's own name for itself.
    pub fn name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Es => "Español",
            Lang::Ca => "Català",
        }
    }

    /// The language's name in `ui`, the language the interface is in.
    pub fn name_in(self, ui: Lang) -> &'static str {
        NAMES[ui as usize][self as usize]
    }

    /// Every language, sorted alphabetically (ignoring accents) by its name in `ui`.
    pub fn sorted_in(ui: Lang) -> Vec<Lang> {
        let mut v = Lang::ALL.to_vec();
        v.sort_by_cached_key(|l| sort_key(l.name_in(ui)));
        v
    }

    /// The Mac's first preferred language among the ones MacControl speaks
    /// (English when it speaks none of them).
    pub fn system() -> Lang {
        if let Ok(l) = std::env::var("MACCONTROL_LANG") {
            return Lang::from_code(&l).unwrap_or(Lang::En);
        }
        let prefs = objc2_foundation::NSLocale::preferredLanguages();
        prefs.iter().find_map(|s| Lang::from_code(&s.to_string())).unwrap_or(Lang::En)
    }

    /// "ca", "es-ES", "en_GB"… → the language, if MacControl has it.
    pub fn from_code(code: &str) -> Option<Lang> {
        let base = code.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
        match base.as_str() {
            "en" => Some(Lang::En),
            "es" => Some(Lang::Es),
            "ca" => Some(Lang::Ca),
            _ => None,
        }
    }
}

/// Language names: one row per interface language, one column per language
/// named, both in the order of `Lang::BY_INDEX` (en es ca).
const NAMES: [[&str; 3]; 3] = [
    ["English", "Spanish", "Catalan"],
    ["Inglés", "Español", "Catalán"],
    ["Anglès", "Espanyol", "Català"],
];

/// Sort key that ignores case and accents, so "Àrab" sorts with the A's.
fn sort_key(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' => 'a',
            'é' | 'è' => 'e',
            'í' | 'ï' => 'i',
            'ó' | 'ò' => 'o',
            'ú' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            c => c,
        })
        .collect()
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set(lang: Lang) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Lang {
    let i = CURRENT.load(Ordering::Relaxed) as usize;
    Lang::BY_INDEX.get(i).copied().unwrap_or(Lang::En)
}

/// English text → its translations, in the order of `Lang::BY_INDEX` after
/// English: Spanish, Catalan.
const TABLE: &[(&str, [&str; 2])] = &[
    // sections
    ("Home", ["Inicio", "Inici"]),
    ("Trackpad gestures", ["Gestos del trackpad", "Gestos del trackpad"]),
    ("Keyboard shortcuts", ["Atajos de teclado", "Dreceres de teclat"]),
    ("Mouse buttons", ["Botones del ratón", "Botons del ratolí"]),
    ("Window snapping", ["Ajuste de ventanas", "Ajust de finestres"]),
    ("Settings", ["Ajustes", "Configuració"]),
    ("Keyboard", ["Teclado", "Teclat"]),
    ("Mouse", ["Ratón", "Ratolí"]),
    ("Windows", ["Ventanas", "Finestres"]),
    ("Everything MacControl can do for you, at a glance.", ["Todo lo que MacControl puede hacer por ti, de un vistazo.", "Tot el que MacControl pot fer per tu, d'un cop d'ull."]),
    ("Create rules: “when I make this gesture with my fingers, MacControl does this”.", ["Crea reglas: «cuando hago este gesto con los dedos, MacControl hace esto».", "Crea regles: «quan faig aquest gest amb els dits, MacControl fa això»."]),
    ("Create rules: “when I press these keys, MacControl does this”, in any app.", ["Crea reglas: «cuando pulso estas teclas, MacControl hace esto», en cualquier app.", "Crea regles: «quan premo aquestes tecles, MacControl fa això», a qualsevol app."]),
    ("Give the extra buttons or the wheel of any mouse a job.", ["Da una función a los botones extra o a la rueda de cualquier ratón.", "Dona una funció als botons extra o a la roda de qualsevol ratolí."]),
    ("Drag a window to an edge or corner of the screen and it snaps into place.", ["Arrastra una ventana a un borde o esquina de la pantalla y se coloca sola.", "Arrossega una finestra a un marge o una cantonada de la pantalla i es col·loca sola."]),
    ("Language, permissions, status and general preferences.", ["Idioma, permisos, estado y preferencias generales.", "Idioma, permisos, estat i preferències generals."]),
    ("Taps, clicks and swipes with 2 to 5 fingers, corners and TipTap.", ["Toques, clics y deslizamientos con 2 a 5 dedos, esquinas y TipTap.", "Tocs, clics i lliscaments amb 2 a 5 dits, cantonades i TipTap."]),
    ("System-wide key combinations that do whatever you want.", ["Combinaciones de teclas globales que hacen lo que quieras.", "Combinacions de tecles globals que fan el que vulguis."]),
    ("Side buttons, middle button and the wheel with keys.", ["Botones laterales, botón central y rueda con teclas.", "Botons laterals, botó central i roda amb tecles."]),
    ("Edges for halves, the top to maximize and corners for quarters.", ["Bordes para mitades, arriba para maximizar y esquinas para cuartos.", "Marges per a meitats, amunt per maximitzar i cantonades per a quarts."]),
    // menus
    ("Open MacControl", ["Abrir MacControl", "Obre MacControl"]),
    ("Pause / resume", ["Pausar / reanudar", "Pausa / reprèn"]),
    ("Quit MacControl", ["Salir de MacControl", "Surt de MacControl"]),
    ("Close MacControl (keeps running in the menu bar)", ["Cerrar MacControl (sigue en la barra de menús)", "Tanca MacControl (continua a la barra de menús)"]),
    ("Close Window", ["Cerrar ventana", "Tanca la finestra"]),
    // sidebar
    ("Your Mac, your way", ["Tu Mac, a tu manera", "El teu Mac, a la teva manera"]),
    ("Active", ["Activo", "Actiu"]),
    ("Paused", ["En pausa", "En pausa"]),
    ("A permission is missing", ["Falta un permiso", "Falta un permís"]),
    ("Listening", ["Escuchando", "Escoltant"]),
    ("Doing nothing", ["No hace nada", "No fa res"]),
    // home
    ("+  New rule", ["+  Nueva regla", "+  Nova regla"]),
    ("Welcome to MacControl", ["Bienvenido a MacControl", "Benvingut a MacControl"]),
    ("Control your Mac with trackpad gestures, keyboard shortcuts and mouse buttons, and place windows just by dragging them to the edges of the screen.", ["Controla tu Mac con gestos del trackpad, atajos de teclado y botones del ratón, y coloca las ventanas solo con arrastrarlas a los bordes de la pantalla.", "Controla el teu Mac amb gestos del trackpad, dreceres de teclat i botons del ratolí, i col·loca les finestres només arrossegant-les als marges de la pantalla."]),
    ("{} active rules", ["{} reglas activas", "{} regles actives"]),
    ("Window snapping on", ["Ajuste de ventanas activado", "Ajust de finestres activat"]),
    ("First step: grant Accessibility permission", ["Primer paso: da permiso de Accesibilidad", "Primer pas: dona permís d'Accessibilitat"]),
    ("Without it, macOS won't let MacControl read gestures and keys or move windows.", ["Sin él, macOS no deja que MacControl lea gestos y teclas ni mueva ventanas.", "Sense aquest permís, macOS no deixa que MacControl llegeixi gestos i tecles ni mogui finestres."]),
    ("Grant permission", ["Conceder permiso", "Concedeix el permís"]),
    ("What do you want to set up?", ["¿Qué quieres configurar?", "Què vols configurar?"]),
    ("On", ["Activado", "Activat"]),
    ("Off", ["Desactivado", "Desactivat"]),
    ("Set up  →", ["Configurar  →", "Configura  →"]),
    ("How it works", ["Cómo funciona", "Com funciona"]),
    ("Choose what you do", ["Elige qué haces", "Tria què fas"]),
    ("A finger gesture, a key combination or a mouse button.", ["Un gesto con los dedos, una combinación de teclas o un botón del ratón.", "Un gest amb els dits, una combinació de tecles o un botó del ratolí."]),
    ("Choose what happens", ["Elige qué pasa", "Tria què passa"]),
    ("Place a window, open an app, send a shortcut, turn the volume up…", ["Colocar una ventana, abrir una app, enviar un atajo, subir el volumen…", "Col·locar una finestra, obrir una app, enviar una drecera, apujar el volum…"]),
    ("That's it", ["Y listo", "I llestos"]),
    ("It works in every app, or only in the ones you choose.", ["Funciona en todas las apps, o solo en las que tú elijas.", "Funciona a totes les apps, o només a les que triïs."]),
    // rules pages
    ("Your rules ({})", ["Tus reglas ({})", "Les teves regles ({})"]),
    ("More ideas", ["Más ideas", "Més idees"]),
    ("Works in", ["Funciona en", "Funciona a"]),
    ("All apps", ["Todas las apps", "Totes les apps"]),
    ("Right-click to remove it", ["Clic derecho para quitarla", "Clic dret per treure-la"]),
    ("Remove this app and its rules", ["Quitar esta app y sus reglas", "Treu aquesta app i les seves regles"]),
    ("+ A specific app", ["+ Una app concreta", "+ Una app concreta"]),
    ("Apps open now", ["Apps abiertas ahora", "Apps obertes ara"]),
    ("Open the app you want and come back here.", ["Abre la app que quieras y vuelve aquí.", "Obre l'app que vulguis i torna aquí."]),
    ("Rules for a specific app take priority over general ones", ["Las reglas de una app concreta mandan sobre las generales", "Les regles d'una app concreta tenen prioritat sobre les generals"]),
    ("You have no rules for all apps yet", ["Aún no tienes reglas para todas las apps", "Encara no tens regles per a totes les apps"]),
    ("You have no rules for {} yet", ["Aún no tienes reglas para {}", "Encara no tens regles per a {}"]),
    ("Start with one of these ideas or create your own.", ["Empieza con una de estas ideas o crea la tuya.", "Comença amb una d'aquestes idees o crea la teva."]),
    ("+  Create a rule", ["+  Crear una regla", "+  Crea una regla"]),
    ("Ideas to get started", ["Ideas para empezar", "Idees per començar"]),
    ("Add this rule", ["Añadir esta regla", "Afegeix aquesta regla"]),
    ("{}-finger tap", ["Toque {} dedos", "Toc {} dits"]),
    ("{}-finger double tap", ["Doble toque {} dedos", "Doble toc {} dits"]),
    ("{}-finger click", ["Clic {} dedos", "Clic {} dits"]),
    ("{} fingers {}", ["{} dedos {}", "{} dits {}"]),
    ("Corner tap", ["Toque en esquina", "Toc a la cantonada"]),
    ("1 finger from edge {}", ["1 dedo desde borde {}", "1 dit des del marge {}"]),
    ("TipTap left", ["TipTap izq.", "TipTap esq."]),
    ("TipTap right", ["TipTap dcha.", "TipTap dta."]),
    ("New shortcut: press a combination", ["Nuevo atajo: pulsa una combinación", "Nova drecera: prem una combinació"]),
    ("Choose what it should do →", ["Elige qué debe hacer →", "Tria què ha de fer →"]),
    ("Try your gestures here", ["Prueba tus gestos aquí", "Prova els teus gestos aquí"]),
    ("This gesture has no rule yet", ["Este gesto aún no tiene regla", "Aquest gest encara no té cap regla"]),
    ("Put your fingers on the trackpad to see which gesture MacControl recognizes.", ["Pon los dedos en el trackpad: verás qué gesto reconoce MacControl.", "Posa els dits al trackpad: veuràs quin gest reconeix MacControl."]),
    ("No trackpad detected.", ["No se ha detectado ningún trackpad.", "No s'ha detectat cap trackpad."]),
    // rule editor
    ("Edit rule", ["Editar regla", "Edita la regla"]),
    ("Enabled", ["Activada", "Activada"]),
    ("Disabled", ["Desactivada", "Desactivada"]),
    ("The gesture you'll make on the trackpad", ["El gesto que harás en el trackpad", "El gest que faràs al trackpad"]),
    ("The key combination you'll press", ["La combinación de teclas que pulsarás", "La combinació de tecles que premeràs"]),
    ("The button or wheel you'll use", ["El botón o la rueda que usarás", "El botó o la roda que faràs servir"]),
    ("When I…", ["Cuando yo…", "Quan jo…"]),
    ("Press the key combination now…", ["Pulsa ahora la combinación de teclas…", "Prem ara la combinació de tecles…"]),
    ("No combination yet", ["Todavía no hay combinación", "Encara no hi ha cap combinació"]),
    ("⚠ On a Mac, fn + {} is the {} key. While this rule is on, {} will stop working in every app.", ["⚠ En el Mac, fn + {} es la tecla {}. Mientras esta regla esté activa, {} dejará de funcionar en todas las apps.", "⚠ Al Mac, fn + {} és la tecla {}. Mentre aquesta regla estigui activa, {} deixarà de funcionar a totes les apps."]),
    ("Cancel", ["Cancelar", "Cancel·la"]),
    ("●  Record combination", ["●  Grabar combinación", "●  Grava la combinació"]),
    ("MacControl will…", ["MacControl hará…", "MacControl farà…"]),
    ("What will happen each time", ["Lo que pasará cada vez", "El que passarà cada vegada"]),
    ("Works in…", ["Funciona en…", "Funciona a…"]),
    ("Every app or just one", ["Todas las apps o solo una", "Totes les apps o només una"]),
    ("Changes are saved automatically.", ["Los cambios se guardan solos.", "Els canvis es desen sols."]),
    ("Delete rule", ["Eliminar regla", "Elimina la regla"]),
    ("Choose another gesture", ["Elegir otro gesto", "Tria un altre gest"]),
    ("click", ["clic", "clic"]),
    ("⌥ option", ["⌥ opción", "⌥ opció"]),
    ("⇧ shift", ["⇧ mayús", "⇧ majúscules"]),
    ("⌘ command", ["⌘ comando", "⌘ ordre"]),
    ("A button", ["Un botón", "Un botó"]),
    ("The wheel + keys", ["La rueda + teclas", "La roda + tecles"]),
    ("Press a mouse button now (middle, side…)", ["Pulsa ahora un botón del ratón (central, lateral…)", "Prem ara un botó del ratolí (central, lateral…)"]),
    ("●  Record button", ["●  Grabar botón", "●  Grava el botó"]),
    ("While holding down (optional)", ["Mientras mantengo pulsadas (opcional)", "Mentre mantinc premudes (opcional)"]),
    ("Wheel up", ["Rueda hacia arriba", "Roda cap amunt"]),
    ("Wheel down", ["Rueda hacia abajo", "Roda cap avall"]),
    ("While holding down at least one of these keys", ["Mientras mantengo pulsada al menos una de estas teclas", "Mentre mantinc premuda almenys una d'aquestes tecles"]),
    ("Choose at least one key: the wheel alone keeps scrolling as usual.", ["Elige al menos una tecla: la rueda sola sigue desplazando como siempre.", "Tria almenys una tecla: la roda sola continua desplaçant com sempre."]),
    ("Nothing yet", ["Nada todavía", "Res encara"]),
    ("Press a keyboard shortcut", ["Pulsar un atajo de teclado", "Prémer una drecera de teclat"]),
    ("Place the window", ["Colocar la ventana", "Col·locar la finestra"]),
    ("A system action", ["Una acción del sistema", "Una acció del sistema"]),
    ("Open an app", ["Abrir una app", "Obrir una app"]),
    ("Open a link or file", ["Abrir un enlace o archivo", "Obrir un enllaç o fitxer"]),
    ("Type some text", ["Escribir un texto", "Escriure un text"]),
    ("Run a command", ["Ejecutar un comando", "Executar una ordre"]),
    ("Choose above what you want to happen.", ["Elige arriba qué quieres que pase.", "Tria a dalt què vols que passi."]),
    ("Press the shortcut MacControl should send now…", ["Pulsa ahora el atajo que MacControl debe enviar…", "Prem ara la drecera que MacControl ha d'enviar…"]),
    ("Record the shortcut to send", ["Graba el atajo que se enviará", "Grava la drecera que s'enviarà"]),
    ("●  Record shortcut", ["●  Grabar atajo", "●  Grava la drecera"]),
    ("App name (for example “Safari” or “Notes”)", ["Nombre de la app (por ejemplo «Safari» o «Notas»)", "Nom de l'app (per exemple «Safari» o «Notes»)"]),
    ("Web address or path to a file or folder", ["Dirección web o ruta de un archivo o carpeta", "Adreça web o camí d'un fitxer o carpeta"]),
    ("Text to type wherever the cursor is", ["Texto que se escribirá donde esté el cursor", "Text que s'escriurà on sigui el cursor"]),
    ("Terminal command (run with sh)", ["Orden de Terminal (se ejecuta con sh)", "Ordre de Terminal (s'executa amb sh)"]),
    ("say hello", ["say hola", "say hola"]),
    ("The active window will go to: {}", ["La ventana activa irá a: {}", "La finestra activa anirà a: {}"]),
    ("L. half", ["Mitad izq.", "Meitat esq."]),
    ("R. half", ["Mitad dcha.", "Meitat dta."]),
    ("Top", ["Arriba", "Dalt"]),
    ("Bottom", ["Abajo", "Baix"]),
    ("Top L.", ["Sup. izq.", "Sup. esq."]),
    ("Top R.", ["Sup. dcha.", "Sup. dta."]),
    ("Bottom L.", ["Inf. izq.", "Inf. esq."]),
    ("Bottom R.", ["Inf. dcha.", "Inf. dta."]),
    ("L. 1/3", ["1/3 izq.", "1/3 esq."]),
    ("Mid 1/3", ["1/3 centro", "1/3 centre"]),
    ("R. 1/3", ["1/3 dcha.", "1/3 dta."]),
    ("L. 2/3", ["2/3 izq.", "2/3 esq."]),
    ("R. 2/3", ["2/3 dcha.", "2/3 dta."]),
    // snapping page
    ("Snap when dragging", ["Ajustar al arrastrar", "Ajusta en arrossegar"]),
    ("Drag a window by its title bar to an edge and let go.", ["Arrastra una ventana por su barra de título hasta un borde y suéltala.", "Arrossega una finestra per la barra de títol fins a un marge i deixa-la anar."]),
    ("Which edges work", ["Qué bordes funcionan", "Quins marges funcionen"]),
    ("◧  Left: half", ["◧  Izquierda: mitad", "◧  Esquerra: meitat"]),
    ("◨  Right: half", ["◨  Derecha: mitad", "◨  Dreta: meitat"]),
    ("⬒  Top: maximize", ["⬒  Arriba: maximizar", "⬒  Dalt: maximitzar"]),
    ("◰  Corners: quarters", ["◰  Esquinas: cuartos", "◰  Cantonades: quarts"]),
    ("Exceptions", ["Excepciones", "Excepcions"]),
    ("Apps whose windows don't snap", ["Apps en las que las ventanas no se ajustan", "Apps en què les finestres no s'ajusten"]),
    ("Remove", ["Quitar", "Treu"]),
    ("+ Add app", ["+ Añadir app", "+ Afegeix una app"]),
    ("Sizes", ["Tamaños", "Mides"]),
    ("How much space the window takes when it snaps", ["Cuánto ocupa la ventana al ajustarse", "Quant ocupa la finestra en ajustar-se"]),
    ("Quarters (corners)", ["Cuartos (esquinas)", "Quarts (cantonades)"]),
    ("Gap around", ["Margen alrededor", "Marge al voltant"]),
    ("Detection", ["Detección", "Detecció"]),
    ("How close to the edge you need to get", ["Qué tan cerca del borde hay que llegar", "Com de prop del marge cal arribar"]),
    ("Corner area", ["Zona de las esquinas", "Zona de les cantonades"]),
    ("Widen the edges", ["Ampliar los bordes", "Amplia els marges"]),
    ("Wait a moment before snapping", ["Esperar un momento antes de ajustar", "Espera un moment abans d'ajustar"]),
    ("Avoids snapping when you brush an edge by accident", ["Evita ajustes al rozar un borde sin querer", "Evita ajustos en fregar un marge sense voler"]),
    ("Behavior", ["Comportamiento", "Comportament"]),
    ("Restore the size when pulled away", ["Recuperar el tamaño al separarla", "Recupera la mida en separar-la"]),
    ("Dragging it off the edge brings back its previous size", ["Al sacarla del borde vuelve a su tamaño de antes", "En treure-la del marge torna a la mida d'abans"]),
    ("Show preview", ["Mostrar vista previa", "Mostra la previsualització"]),
    ("A rectangle shows where the window will land", ["Un rectángulo indica dónde quedará la ventana", "Un rectangle indica on quedarà la finestra"]),
    ("Animation", ["Animación", "Animació"]),
    ("Restore the defaults", ["Volver a los valores originales", "Torna als valors originals"]),
    ("Hover over the colored zones to see where the window would go.", ["Pasa el ratón por las zonas de color para ver dónde quedaría la ventana.", "Passa el ratolí per les zones de color per veure on quedaria la finestra."]),
    ("Window snapped: {}", ["Ventana ajustada: {}", "Finestra ajustada: {}"]),
    // settings page
    ("Language", ["Idioma", "Idioma"]),
    ("Permissions", ["Permisos", "Permisos"]),
    ("Accessibility", ["Accesibilidad", "Accessibilitat"]),
    ("Granted", ["Concedido", "Concedit"]),
    ("Pending", ["Pendiente", "Pendent"]),
    ("MacControl needs it to read shortcuts and mouse buttons and to move other apps' windows.", ["MacControl lo necesita para leer atajos, botones del ratón y para mover las ventanas de otras apps.", "MacControl el necessita per llegir dreceres i botons del ratolí i per moure les finestres d'altres apps."]),
    ("Request permission", ["Pedir permiso", "Demana el permís"]),
    ("Open System Settings", ["Abrir Ajustes del Sistema", "Obre la Configuració del Sistema"]),
    ("Status", ["Estado", "Estat"]),
    ("Keyboard and mouse", ["Teclado y ratón", "Teclat i ratolí"]),
    ("Stopped", ["Detenido", "Aturat"]),
    ("1 detected", ["1 detectado", "1 detectat"]),
    ("Several detected", ["Varios detectados", "Diversos detectats"]),
    ("None", ["Ninguno", "Cap"]),
    ("Frontmost app", ["App en primer plano", "App en primer pla"]),
    ("MacControl active", ["MacControl activo", "MacControl actiu"]),
    ("Pauses every rule and window snapping", ["Pausa todas las reglas y el ajuste de ventanas", "Posa en pausa totes les regles i l'ajust de finestres"]),
    ("Click sound", ["Sonido al hacer clic", "So en fer clic"]),
    ("A little “beep” when you click inside MacControl", ["Un pequeño «bip» al pulsar dentro de MacControl", "Un petit «bip» en fer clic dins de MacControl"]),
    ("When you close the window, MacControl keeps working from its icon in the menu bar.", ["Al cerrar la ventana, MacControl sigue funcionando desde su icono en la barra de menús.", "En tancar la finestra, MacControl continua funcionant des de la seva icona a la barra de menús."]),
    ("Settings saved in", ["Configuración guardada en", "Configuració desada a"]),
    ("Are you sure? All your rules will be deleted.", ["¿Seguro? Se borrarán todas tus reglas.", "Segur? S'esborraran totes les teves regles."]),
    ("Yes, reset", ["Sí, restablecer", "Sí, restableix"]),
    ("Reset everything", ["Restablecer todo", "Restableix-ho tot"]),
    ("About", ["Acerca de", "Quant a"]),
    ("Author", ["Autor", "Autor"]),
    ("Contact", ["Contacto", "Contacte"]),
    ("License", ["Licencia", "Llicència"]),
    ("Click sound: “audio-volume-change” by Lucas McCallister, from the freedesktop.org sound theme (CC-BY-SA license).", ["Sonido de clic: «audio-volume-change», de Lucas McCallister, del tema de sonidos de freedesktop.org (licencia CC-BY-SA).", "So de clic: «audio-volume-change», de Lucas McCallister, del tema de sons de freedesktop.org (llicència CC-BY-SA)."]),
    ("Version {}", ["Versión {}", "Versió {}"]),
    ("MacControl is a free app for smart management of the trackpad, keyboard shortcuts and window snapping on macOS.", ["MacControl es una app gratuita para la gestión inteligente del trackpad, los atajos de teclado y el ajuste de ventanas en macOS.", "MacControl és una app gratuïta per a la gestió intel·ligent del trackpad, les dreceres de teclat i l'ajust de finestres a macOS."]),
    // keys, gestures, buttons
    ("Space", ["Espacio", "Espai"]),
    ("Key {}", ["Tecla {}", "Tecla {}"]),
    ("Page Up", ["RePág", "RePàg"]),
    ("End", ["Fin", "Fi"]),
    ("Page Down", ["AvPág", "AvPàg"]),
    ("up", ["arriba", "amunt"]),
    ("down", ["abajo", "avall"]),
    ("left", ["izquierda", "a l'esquerra"]),
    ("right", ["derecha", "a la dreta"]),
    ("top left", ["superior izquierda", "superior esquerra"]),
    ("top right", ["superior derecha", "superior dreta"]),
    ("bottom left", ["inferior izquierda", "inferior esquerra"]),
    ("bottom right", ["inferior derecha", "inferior dreta"]),
    ("Tap with {} fingers", ["Toque con {} dedos", "Toc amb {} dits"]),
    ("Double tap with {} fingers", ["Doble toque con {} dedos", "Doble toc amb {} dits"]),
    ("Click with {} fingers", ["Clic con {} dedos", "Clic amb {} dits"]),
    ("Swipe {} fingers {}", ["Deslizar {} dedos hacia {}", "Lliscar {} dits cap {}"]),
    ("1-finger tap in the {} corner", ["Toque de 1 dedo en esquina {}", "Toc d'1 dit a la cantonada {}"]),
    ("Swipe 1 finger in from the left edge", ["Deslizar 1 dedo desde el borde izquierdo", "Lliscar 1 dit des del marge esquerre"]),
    ("Swipe 1 finger in from the right edge", ["Deslizar 1 dedo desde el borde derecho", "Lliscar 1 dit des del marge dret"]),
    ("Swipe 1 finger in from the top edge", ["Deslizar 1 dedo desde el borde superior", "Lliscar 1 dit des del marge superior"]),
    ("Swipe 1 finger in from the bottom edge", ["Deslizar 1 dedo desde el borde inferior", "Lliscar 1 dit des del marge inferior"]),
    ("TipTap left (1 finger resting)", ["TipTap izquierda (1 dedo apoyado)", "TipTap esquerra (1 dit recolzat)"]),
    ("TipTap right (1 finger resting)", ["TipTap derecha (1 dedo apoyado)", "TipTap dreta (1 dit recolzat)"]),
    ("Taps", ["Toques", "Tocs"]),
    ("Clicks", ["Clics", "Clics"]),
    ("Swipes", ["Deslizar", "Lliscaments"]),
    ("Corners", ["Esquinas", "Cantonades"]),
    ("From the edge", ["Desde el borde", "Des del marge"]),
    ("Middle button", ["Botón central", "Botó central"]),
    ("Button 4 (back)", ["Botón 4 (atrás)", "Botó 4 (enrere)"]),
    ("Button 5 (forward)", ["Botón 5 (adelante)", "Botó 5 (endavant)"]),
    ("Button {}", ["Botón {}", "Botó {}"]),
    ("{} + wheel up", ["{} + rueda hacia arriba", "{} + roda cap amunt"]),
    ("{} + wheel down", ["{} + rueda hacia abajo", "{} + roda cap avall"]),
    // window positions
    ("Maximize", ["Maximizar", "Maximitzar"]),
    ("Left half", ["Mitad izquierda", "Meitat esquerra"]),
    ("Right half", ["Mitad derecha", "Meitat dreta"]),
    ("Top half", ["Mitad superior", "Meitat superior"]),
    ("Bottom half", ["Mitad inferior", "Meitat inferior"]),
    ("Top-left quarter", ["Cuarto superior izquierdo", "Quart superior esquerre"]),
    ("Top-right quarter", ["Cuarto superior derecho", "Quart superior dret"]),
    ("Bottom-left quarter", ["Cuarto inferior izquierdo", "Quart inferior esquerre"]),
    ("Bottom-right quarter", ["Cuarto inferior derecho", "Quart inferior dret"]),
    ("Left third", ["Tercio izquierdo", "Terç esquerre"]),
    ("Center third", ["Tercio central", "Terç central"]),
    ("Right third", ["Tercio derecho", "Terç dret"]),
    ("Left two thirds", ["Dos tercios izquierda", "Dos terços esquerra"]),
    ("Right two thirds", ["Dos tercios derecha", "Dos terços dreta"]),
    ("Center", ["Centrar", "Centrar"]),
    // system actions
    ("App windows (App Exposé)", ["Ventanas de la app (App Exposé)", "Finestres de l'app (App Exposé)"]),
    ("Show desktop", ["Mostrar escritorio", "Mostrar l'escriptori"]),
    ("Lock screen", ["Bloquear pantalla", "Bloquejar la pantalla"]),
    ("Capture an area", ["Captura de una zona", "Captura d'una zona"]),
    ("Volume up", ["Subir volumen", "Apujar el volum"]),
    ("Volume down", ["Bajar volumen", "Abaixar el volum"]),
    ("Mute / unmute", ["Silenciar / activar sonido", "Silenciar / activar el so"]),
    ("Minimize window", ["Minimizar ventana", "Minimitzar la finestra"]),
    ("Minimize the window under the cursor", ["Minimizar la ventana bajo el cursor", "Minimitzar la finestra sota el cursor"]),
    ("Close window", ["Cerrar ventana", "Tancar la finestra"]),
    ("Full screen", ["Pantalla completa", "Pantalla completa"]),
    ("Restore previous size", ["Restaurar tamaño anterior", "Restaurar la mida anterior"]),
    ("Move to the next screen", ["Mover a la siguiente pantalla", "Moure a la pantalla següent"]),
    ("Left click", ["Clic izquierdo", "Clic esquerre"]),
    ("Right click", ["Clic derecho", "Clic dret"]),
    ("Middle click", ["Clic central", "Clic central"]),
    ("System", ["Sistema", "Sistema"]),
    ("Sound", ["Sonido", "So"]),
    // actions
    ("No action", ["Sin acción", "Sense acció"]),
    ("Press {}", ["Pulsar {}", "Prémer {}"]),
    ("Open {}", ["Abrir {}", "Obrir {}"]),
    ("link…", ["enlace…", "enllaç…"]),
    ("Type “{}”", ["Escribir «{}»", "Escriure «{}»"]),
    ("Run “{}”", ["Ejecutar «{}»", "Executar «{}»"]),
];

fn table() -> &'static HashMap<&'static str, [&'static str; 2]> {
    static MAP: OnceLock<HashMap<&'static str, [&'static str; 2]>> = OnceLock::new();
    MAP.get_or_init(|| TABLE.iter().map(|(en, tr)| (*en, *tr)).collect())
}

/// The text in the current language.
pub fn tl(en: &'static str) -> &'static str {
    match current() {
        Lang::En => en,
        lang => table().get(en).and_then(|tr| tr.get(lang as usize - 1)).copied().unwrap_or(en),
    }
}

/// `tl` for texts with `{}` placeholders, filled in order.
pub fn tlf(en: &'static str, args: &[&dyn std::fmt::Display]) -> String {
    let mut out = String::new();
    let mut rest = tl(en);
    for a in args {
        match rest.find("{}") {
            Some(i) => {
                out.push_str(&rest[..i]);
                out.push_str(&a.to_string());
                rest = &rest[i + 2..];
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

/// Draws the language's flag filling `r`.
pub fn paint_flag(p: &Painter, r: Rect, lang: Lang) {
    let p = p.with_clip_rect(r.intersect(p.clip_rect()));
    let (w, h) = (r.width(), r.height());
    match lang {
        Lang::Ca => {
            // the senyera: nine stripes, four of them red (FLAG's height is
            // a multiple of 9, so each stripe is a whole number of pixels)
            p.rect_filled(r, 0.0, Color32::from_rgb(252, 221, 9));
            let s = h / 9.0;
            for k in [1.0, 3.0, 5.0, 7.0] {
                p.rect_filled(Rect::from_min_size(pos2(r.min.x, r.min.y + k * s), vec2(w, s)), 0.0, Color32::from_rgb(218, 18, 26));
            }
        }
        Lang::Es => {
            p.rect_filled(r, 0.0, Color32::from_rgb(170, 21, 27));
            p.rect_filled(Rect::from_min_size(pos2(r.min.x, r.min.y + h / 4.0), vec2(w, h / 2.0)), 0.0, Color32::from_rgb(241, 191, 0));
        }
        Lang::En => {
            // the Union Flag, simplified for small sizes
            let blue = Color32::from_rgb(1, 33, 105);
            let red = Color32::from_rgb(200, 16, 46);
            p.rect_filled(r, 0.0, blue);
            let u = h / 30.0;
            for (a, b) in [(r.left_top(), r.right_bottom()), (r.left_bottom(), r.right_top())] {
                p.line_segment([a, b], Stroke::new(6.0 * u, Color32::WHITE));
                p.line_segment([a, b], Stroke::new(2.0 * u, red));
            }
            let c = r.center();
            p.rect_filled(Rect::from_center_size(c, vec2(w, 10.0 * u)), 0.0, Color32::WHITE);
            p.rect_filled(Rect::from_center_size(c, vec2(10.0 * u, h)), 0.0, Color32::WHITE);
            p.rect_filled(Rect::from_center_size(c, vec2(w, 6.0 * u)), 0.0, red);
            p.rect_filled(Rect::from_center_size(c, vec2(6.0 * u, h)), 0.0, red);
        }
    }
    p.rect_stroke(r, 0.0, Stroke::new(0.5_f32, Color32::from_black_alpha(60)), egui::StrokeKind::Inside);
}

/// Flag size: 3:2, and 18 points high so the senyera's nine stripes are 2
/// points (4 pixels on Retina) each and stay crisp.
const FLAG: egui::Vec2 = egui::Vec2::new(27.0, 18.0);

/// Height of the language button.
pub const PICKER_H: f32 = 36.0;
/// Space between the button's edges and its flag / chevron.
const PICKER_PAD: f32 = 14.0;

/// Where a flag goes: `FLAG`-sized, `left` from the left and centred on
/// `cy`, snapped to whole pixels so the stripes' edges stay sharp.
fn flag_rect(ui: &egui::Ui, left: f32, cy: f32) -> Rect {
    let ppp = ui.ctx().pixels_per_point();
    let snap = |v: f32| (v * ppp).round() / ppp;
    Rect::from_min_size(pos2(snap(left), snap(cy - FLAG.y / 2.0)), FLAG)
}

/// A button showing the current language's flag and name that opens a menu
/// with all of them. Returns true when the language changed.
pub fn lang_picker(ui: &mut egui::Ui, lang: &mut Lang) -> bool {
    use crate::ui::theme::{glass, MUTED};
    let label = lang.name();
    let font = FontId::proportional(13.0);
    let text_w = ui.painter().layout_no_wrap(label.to_string(), font.clone(), Color32::WHITE).size().x;
    let (r, resp) = ui.allocate_exact_size(vec2(PICKER_PAD + FLAG.x + 8.0 + text_w + 10.0 + 8.0 + PICKER_PAD, PICKER_H), Sense::click());
    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    let text_col = ui.visuals().text_color();
    // a glass button that lightens under the pointer, with keyboard focus and
    // while its menu is open
    let lit = resp.hovered() || resp.has_focus() || egui::Popup::is_id_open(ui.ctx(), egui::Popup::default_response_id(&resp));
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("hov"), lit, 0.15);
    let p = ui.painter();
    let down = if resp.is_pointer_button_down_on() { 8.0 } else { 0.0 };
    p.rect_filled(r, 10.0, glass((16.0 + 14.0 * hov + down) as u8));
    p.rect_stroke(r, 10.0, Stroke::new(1.0, glass((28.0 + 30.0 * hov) as u8)), egui::StrokeKind::Inside);
    let fr = flag_rect(ui, r.min.x + PICKER_PAD, r.center().y);
    paint_flag(p, fr, *lang);
    p.text(pos2(fr.max.x + 8.0, r.center().y), Align2::LEFT_CENTER, label, font.clone(), text_col);
    // a small chevron
    let c = pos2(r.max.x - PICKER_PAD - 4.0, r.center().y);
    p.line_segment([c + vec2(-3.5, -1.5), c + vec2(0.0, 2.0)], Stroke::new(1.3_f32, text_col));
    p.line_segment([c + vec2(0.0, 2.0), c + vec2(3.5, -1.5)], Stroke::new(1.3_f32, text_col));
    let resp = resp.on_hover_text(tl("Language"));

    let mut changed = false;
    egui::Popup::menu(&resp).show(|ui| {
        // names in the interface language, alphabetically; each language's
        // own name in grey on the right, so it can be found from any language
        let here = current();
        let small = FontId::proportional(12.0);
        for l in Lang::sorted_in(here) {
            let (rr, rsp) = ui.allocate_exact_size(vec2(260.0, 32.0), Sense::click());
            let rsp = rsp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let hov = ui.ctx().animate_bool_with_time(rsp.id.with("hov"), rsp.hovered() || rsp.has_focus(), 0.12);
            let p = ui.painter();
            if hov > 0.0 {
                p.rect_filled(rr, 7.0, glass((24.0 * hov) as u8));
            }
            let fr = flag_rect(ui, rr.min.x + 10.0, rr.center().y);
            paint_flag(p, fr, l);
            let col = ui.visuals().text_color();
            let name = l.name_in(here);
            p.text(pos2(fr.max.x + 10.0, rr.center().y), Align2::LEFT_CENTER, name, font.clone(), col);
            if !name.to_lowercase().starts_with(&l.name().to_lowercase()) {
                p.text(pos2(rr.max.x - 32.0, rr.center().y), Align2::RIGHT_CENTER, l.name(), small.clone(), MUTED);
            }
            if l == *lang {
                p.text(pos2(rr.max.x - 12.0, rr.center().y), Align2::RIGHT_CENTER, "✓", font.clone(), col);
            }
            if rsp.clicked() && l != *lang {
                *lang = l;
                changed = true;
            }
        }
    });
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_keep_placeholders_and_fall_back_to_english() {
        let mut seen = std::collections::HashSet::new();
        for (en, tr) in TABLE {
            assert!(seen.insert(*en), "duplicated: {en}");
            for x in tr {
                assert_eq!(en.matches("{}").count(), x.matches("{}").count(), "{en} → {x}");
                assert!(!x.is_empty(), "{en}");
            }
        }
        for (i, l) in Lang::BY_INDEX.iter().enumerate() {
            assert_eq!(*l as usize, i);
        }
        set(Lang::Es);
        assert_eq!(tl("Settings"), "Ajustes");
        assert_eq!(tlf("Version {}", &[&"1.0"]), "Versión 1.0");
        assert_eq!(tl("not in the table"), "not in the table");
        set(Lang::Ca);
        assert_eq!(tl("Settings"), "Configuració");
        assert_eq!(tlf("Swipe {} fingers {}", &[&3, &tl("left")]), "Lliscar 3 dits cap a l'esquerra");
        set(Lang::En);
        assert_eq!(tl("Settings"), "Settings");
    }

    /// Every text the code passes to `tl`/`tlf` is in the table, so no
    /// screen is left half in English by a typo.
    #[test]
    fn every_text_in_the_code_is_translated() {
        // the same in every language, so not in the table
        let same = ["Trackpad", "Trackpads", "General", "Color", "MacControl", "Mission Control", "Launchpad", "TipTap", "app…", "⌃ control"];
        let sources = [
            include_str!("ui/mod.rs"),
            include_str!("ui/pages.rs"),
            include_str!("model.rs"),
            include_str!("mac/dock.rs"),
            include_str!("mac/snapper.rs"),
        ];
        let mut missing = vec![];
        for src in sources {
            for call in ["tl(\"", "tlf(\""] {
                for (i, _) in src.match_indices(call) {
                    let rest = &src[i + call.len()..];
                    let Some(end) = rest.find('"') else { continue };
                    let key = &rest[..end];
                    if !same.contains(&key) && !TABLE.iter().any(|(en, _)| *en == key) {
                        missing.push(key.to_string());
                    }
                }
            }
        }
        assert!(missing.is_empty(), "not in the table: {missing:#?}");
    }

    #[test]
    fn language_names_and_codes() {
        for l in Lang::ALL {
            assert_eq!(l.name_in(l), l.name());
        }
        assert_eq!(Lang::sorted_in(Lang::En), vec![Lang::Ca, Lang::En, Lang::Es]);
        assert_eq!(Lang::sorted_in(Lang::Ca), vec![Lang::En, Lang::Ca, Lang::Es]);
        assert_eq!(Lang::from_code("ca-ES"), Some(Lang::Ca));
        assert_eq!(Lang::from_code("es_419"), Some(Lang::Es));
        assert_eq!(Lang::from_code("en-GB"), Some(Lang::En));
        assert_eq!(Lang::from_code("fr"), None);
    }
}

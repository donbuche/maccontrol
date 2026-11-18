export default {
  lang: 'es',
  locale: 'es_ES',
  name: 'Español',

  meta: {
    title: 'MacControl: gestos del trackpad, atajos y ajuste de ventanas para Mac',
    description:
      'App gratuita para Mac: da nuevas funciones a los gestos del trackpad, los atajos de teclado y los botones del ratón, y coloca las ventanas arrastrándolas a los bordes de la pantalla.',
    ogTitle: 'MacControl: tu Mac, a tu manera',
    ogAlt: 'MacControl: una chistera negra sobre un fondo cian junto a la pantalla de inicio de la app',
  },

  skip: 'Ir al contenido',
  nav: {
    label: 'Principal',
    features: 'Funciones',
    demo: 'Pruébalo',
    gallery: 'Capturas',
    install: 'Instalar',
    faq: 'Preguntas',
    download: 'Descargar',
    menu: 'Menú',
    language: 'Idioma',
  },

  hero: {
    eyebrow: 'Gratis para macOS · v{version}',
    title: 'Tu Mac, <em>a&nbsp;tu&nbsp;manera</em>',
    lead:
      'Da nuevas funciones a los gestos del trackpad, los atajos de teclado y los botones del ratón, y coloca las ventanas solo con arrastrarlas a los bordes de la pantalla. Una app pequeña que vive en tu barra de menús.',
    ctaPrimary: 'Descargar para Mac',
    ctaSecondary: 'Ver en GitHub',
    points: ['Gratis, sin cuentas', 'Nada sale de tu Mac', 'Funciona en todas las apps'],
    imageAlt: 'Pantalla de inicio de MacControl: accesos a gestos del trackpad, atajos de teclado, botones del ratón y ajuste de ventanas',
    chipA: { keys: '⌃⌥←', text: 'Mitad izquierda' },
    chipB: { keys: '3 dedos', text: 'Clic central' },
  },

  ticker: [
    'Toque con 3 dedos → Clic central',
    '⌃⌥← → Mitad izquierda',
    'Botón lateral → Atrás',
    'Toque con 4 dedos → Launchpad',
    'Arrastrar arriba → Maximizar',
    'TipTap izquierda → ⌘[',
    '⌥ + clic central → Mission Control',
    'Toque en una esquina → Mostrar escritorio',
  ],

  features: {
    kicker: 'Cuatro trucos en la chistera',
    title: 'Todo lo que tocas, un poco más listo',
    intro: 'Cada regla es una frase: «cuando hago esto, MacControl hace aquello». Haz que funcione en todas las apps o solo en las que elijas.',
    items: [
      {
        icon: 'trackpad', tone: 'violet', title: 'Gestos del trackpad',
        text: 'Toques, dobles toques y clics con 2 a 5 dedos, deslizamientos con 3, 4 y 5 dedos, toques con un dedo en las esquinas, deslizamientos desde el borde y TipTap.',
        example: { keys: '3 dedos', text: 'Clic central' },
      },
      {
        icon: 'keyboard', tone: 'cyan', title: 'Atajos de teclado',
        text: 'Atajos globales que hacen lo que quieras: colocar la ventana, abrir una app o un enlace, escribir un texto o ejecutar un comando.',
        example: { keys: '⌃⌥↩', text: 'Maximizar' },
      },
      {
        icon: 'mouse', tone: 'pink', title: 'Botones del ratón',
        text: 'Botones laterales, botón central y rueda combinada con teclas. Atrás y adelante en el navegador, de serie.',
        example: { keys: 'Botón 4', text: 'Atrás' },
      },
      {
        icon: 'window', tone: 'mint', title: 'Ajuste de ventanas',
        text: 'Arrastra una ventana al borde izquierdo o derecho para ocupar media pantalla, arriba para maximizarla y a una esquina para un cuarto. Una vista previa te enseña dónde quedará.',
        example: { keys: 'Arrastrar ←', text: 'Mitad izquierda' },
      },
    ],
  },

  demo: {
    kicker: 'Pruébalo',
    title: 'Arrastra la ventana a un borde',
    intro: 'Así funciona el ajuste de ventanas. Arrastra la ventana por su barra de título hasta un borde o una esquina de esta pantalla y suéltala. O usa los botones y los atajos de abajo.',
    windowTitle: 'Notas',
    windowText: 'Arrástrame por la barra de título',
    hint: 'Arrastra a un borde o una esquina',
    keys: 'Teclado: <kbd>⌃</kbd><kbd>⌥</kbd><kbd>←</kbd> izquierda · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>→</kbd> derecha · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>↩</kbd> maximizar · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>C</kbd> centrar',
    buttons: 'Colocar la ventana',
    positions: {
      left: 'Mitad izquierda', right: 'Mitad derecha', max: 'Maximizar', center: 'Centrar',
      tl: 'Cuarto superior izquierdo', tr: 'Cuarto superior derecho', bl: 'Cuarto inferior izquierdo', br: 'Cuarto inferior derecho',
    },
    moved: 'Ventana colocada: {pos}',
  },

  how: {
    kicker: 'Cómo funciona',
    title: 'Tres pasos, sin manual',
    steps: [
      { title: 'Elige qué haces', text: 'Un gesto con los dedos, una combinación de teclas o un botón del ratón. Lo grabas con solo hacerlo.' },
      { title: 'Elige qué pasa', text: 'Colocar la ventana, abrir una app, enviar un atajo, subir el volumen… Lo eliges de una lista.' },
      { title: 'Y listo', text: 'Funciona en todas las apps, o solo en las que tú elijas. Los cambios se guardan solos.' },
    ],
  },

  actions: {
    kicker: 'Lo que MacControl sabe hacer',
    title: 'Un gesto, muchos trucos',
    groups: [
      { title: 'Ventanas', items: ['Mitades, tercios y cuartos', 'Maximizar y centrar', 'Mover a la siguiente pantalla', 'Restaurar el tamaño anterior', 'Minimizar, cerrar, pantalla completa'] },
      { title: 'Sistema', items: ['Mission Control y App Exposé', 'Mostrar escritorio y Launchpad', 'Bloquear pantalla y captura', 'Subir, bajar y silenciar el volumen', 'Clic izquierdo, derecho y central'] },
      { title: 'A tu medida', items: ['Enviar cualquier atajo de teclado', 'Abrir una app', 'Abrir una web o un archivo', 'Escribir un texto', 'Ejecutar un comando de terminal'] },
    ],
  },

  gallery: {
    kicker: 'Capturas',
    title: 'Echa un vistazo',
    open: 'Abrir captura: {caption}',
    close: 'Cerrar',
    prev: 'Captura anterior',
    next: 'Captura siguiente',
    items: {
      home: 'Inicio: todo de un vistazo',
      trackpad: 'Gestos del trackpad, con un probador en directo',
      keyboard: 'Atajos de teclado',
      mouse: 'Botones del ratón y rueda',
      snapping: 'Ajuste de ventanas y sus opciones',
      settings: 'Ajustes, permisos y estado',
    },
  },

  install: {
    kicker: 'Instalar',
    title: 'En marcha en dos minutos',
    steps: [
      'Descarga el archivo <strong>.dmg</strong> de la última versión en GitHub y ábrelo.',
      'Arrastra <strong>MacControl</strong> a tu carpeta <strong>Aplicaciones</strong>.',
      'La primera vez, macOS dirá que no puede comprobar la app. Ve a <strong>Ajustes del Sistema → Privacidad y seguridad</strong> y pulsa <strong>Abrir igualmente</strong>.',
      'Permite MacControl en <strong>Privacidad y seguridad → Accesibilidad</strong> (y en <strong>Monitorización de entrada</strong> si te lo pide). Lo necesita para leer los atajos y mover las ventanas.',
    ],
    note: 'Requiere macOS 12 o posterior en un Mac con chip de Apple.',
    cta: 'Descargar la última versión',
  },

  faq: {
    kicker: 'Preguntas',
    title: 'Preguntas frecuentes',
    items: [
      { q: '¿MacControl es gratis?', a: 'Sí. Es gratis de descargar y de usar, sin cuentas, sin anuncios y sin compras dentro de la app.' },
      { q: '¿Por qué necesita el permiso de Accesibilidad?', a: 'Para detectar tus atajos y los botones del ratón en cualquier app, y para mover y cambiar el tamaño de las ventanas de otras apps. macOS pide este permiso a cualquier app que haga estas cosas.' },
      { q: '¿Registra lo que escribo?', a: 'No. MacControl solo compara cada tecla con tus reglas y deja pasar el resto. No guarda ni envía nada, y el código está en GitHub para que cualquiera lo lea.' },
      { q: 'macOS dice que no puede abrir la app. ¿Qué hago?', a: 'MacControl todavía no está notarizada por Apple. Ve a <strong>Ajustes del Sistema → Privacidad y seguridad</strong> y pulsa <strong>Abrir igualmente</strong> junto al mensaje sobre MacControl. Solo hay que hacerlo una vez.' },
      { q: '¿En qué Macs funciona?', a: 'En Macs con chip de Apple (M1 o posterior) con macOS 12 Monterey o posterior. Los gestos necesitan el trackpad del portátil o un Magic Trackpad.' },
      { q: '¿Qué pasa al cerrar la ventana?', a: 'MacControl sigue funcionando desde su icono de chistera en la barra de menús, y su icono del Dock desaparece. Para cerrarla del todo, elige <strong>Salir de MacControl</strong> en el menú de ese icono.' },
      { q: '¿En qué idioma está la app?', a: 'En castellano. Los atajos y los gestos funcionan igual sea cual sea el idioma de tu Mac.' },
    ],
  },

  cta: {
    title: '¿Listo para un poco de magia?',
    text: 'Descarga MacControl y haz que tu Mac trabaje como tú.',
    button: 'Descargar para Mac',
  },

  footer: {
    tagline: 'Gestos del trackpad, atajos de teclado, botones del ratón y ajuste de ventanas para macOS.',
    madeBy: 'Diseñada y desarrollada con {heart} en Arenys de Munt por {author}.',
    disclaimer: 'Mac, macOS, Magic Trackpad y Mission Control son marcas de Apple Inc. MacControl es una app independiente, sin relación con Apple ni respaldada por ella.',
    links: 'Enlaces',
    source: 'Código fuente',
    releases: 'Versiones',
    cookies: 'Preferencias de cookies',
    languages: 'Idiomas',
  },

  consent: {
    title: 'Cookies',
    text: 'Usamos cookies de Google Analytics para contar las visitas y ver qué partes de esta página son útiles. Solo se instalan si las aceptas.',
    accept: 'Aceptar',
    reject: 'Rechazar',
  },

  suggest: {
    text: 'Esta página también está en español.',
    cta: 'Ver en español',
    dismiss: 'Cerrar',
  },

  notFound: {
    title: 'No hay nada en esta chistera',
    text: 'La página que buscas no está aquí. Puede que se haya movido o que nunca existiera.',
    back: 'Volver al inicio',
    metaTitle: 'Página no encontrada · MacControl',
  },
};

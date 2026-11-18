export default {
  lang: 'ca',
  locale: 'ca_ES',
  name: 'Català',

  meta: {
    title: 'MacControl: gestos del trackpad, dreceres i ajust de finestres per a Mac',
    description:
      'App gratuïta per a Mac: dona noves funcions als gestos del trackpad, les dreceres de teclat i els botons del ratolí, i col·loca les finestres arrossegant-les als marges de la pantalla.',
    ogTitle: 'MacControl: el teu Mac, a la teva manera',
    ogAlt: 'MacControl: un barret de copa negre sobre un fons cian al costat de la pantalla d’inici de l’app',
  },

  skip: 'Ves al contingut',
  nav: {
    label: 'Principal',
    features: 'Funcions',
    demo: 'Prova-ho',
    gallery: 'Captures',
    install: 'Instal·lar',
    faq: 'Preguntes',
    download: 'Descarregar',
    menu: 'Menú',
    language: 'Idioma',
  },

  hero: {
    eyebrow: 'Gratuïta per a macOS · v{version}',
    title: 'El teu Mac, <em>a la teva&nbsp;manera</em>',
    lead:
      'Dona noves funcions als gestos del trackpad, les dreceres de teclat i els botons del ratolí, i col·loca les finestres només arrossegant-les als marges de la pantalla. Una app petita que viu a la barra de menús.',
    ctaPrimary: 'Descarregar per a Mac',
    ctaSecondary: 'Veure a GitHub',
    points: ['Gratuïta, sense comptes', 'Res no surt del teu Mac', 'Funciona a totes les apps'],
    imageAlt: 'Pantalla d’inici de MacControl: accessos a gestos del trackpad, dreceres de teclat, botons del ratolí i ajust de finestres',
    chipA: { keys: '⌃⌥←', text: 'Meitat esquerra' },
    chipB: { keys: '3 dits', text: 'Clic central' },
  },

  ticker: [
    'Toc amb 3 dits → Clic central',
    '⌃⌥← → Meitat esquerra',
    'Botó lateral → Enrere',
    'Toc amb 4 dits → Launchpad',
    'Arrossegar amunt → Maximitzar',
    'TipTap esquerra → ⌘[',
    '⌥ + clic central → Mission Control',
    'Toc a una cantonada → Mostrar l’escriptori',
  ],

  features: {
    kicker: 'Quatre trucs dins el barret',
    title: 'Tot el que toques, una mica més llest',
    intro: 'Cada regla és una frase: «quan faig això, MacControl fa allò». Fes que funcioni a totes les apps o només a les que triïs.',
    items: [
      {
        icon: 'trackpad', tone: 'violet', title: 'Gestos del trackpad',
        text: 'Tocs, dobles tocs i clics amb 2 a 5 dits, lliscaments amb 3, 4 i 5 dits, tocs amb un dit a les cantonades, lliscaments des del marge i TipTap.',
        example: { keys: '3 dits', text: 'Clic central' },
      },
      {
        icon: 'keyboard', tone: 'cyan', title: 'Dreceres de teclat',
        text: 'Dreceres globals que fan el que vulguis: col·locar la finestra, obrir una app o un enllaç, escriure un text o executar una ordre.',
        example: { keys: '⌃⌥↩', text: 'Maximitzar' },
      },
      {
        icon: 'mouse', tone: 'pink', title: 'Botons del ratolí',
        text: 'Botons laterals, botó central i roda combinada amb tecles. Enrere i endavant al navegador, de sèrie.',
        example: { keys: 'Botó 4', text: 'Enrere' },
      },
      {
        icon: 'window', tone: 'mint', title: 'Ajust de finestres',
        text: 'Arrossega una finestra al marge esquerre o dret per ocupar mitja pantalla, amunt per maximitzar-la i a una cantonada per a un quart. Una previsualització t’ensenya on quedarà.',
        example: { keys: 'Arrossegar ←', text: 'Meitat esquerra' },
      },
    ],
  },

  demo: {
    kicker: 'Prova-ho',
    title: 'Arrossega la finestra a un marge',
    intro: 'Així funciona l’ajust de finestres. Arrossega la finestra per la barra de títol fins a un marge o una cantonada d’aquesta pantalla i deixa-la anar. O fes servir els botons i les dreceres de sota.',
    windowTitle: 'Notes',
    windowText: 'Arrossega’m per la barra de títol',
    hint: 'Arrossega a un marge o una cantonada',
    keys: 'Teclat: <kbd>⌃</kbd><kbd>⌥</kbd><kbd>←</kbd> esquerra · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>→</kbd> dreta · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>↩</kbd> maximitzar · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>C</kbd> centrar',
    buttons: 'Col·locar la finestra',
    positions: {
      left: 'Meitat esquerra', right: 'Meitat dreta', max: 'Maximitzar', center: 'Centrar',
      tl: 'Quart superior esquerre', tr: 'Quart superior dret', bl: 'Quart inferior esquerre', br: 'Quart inferior dret',
    },
    moved: 'Finestra col·locada: {pos}',
  },

  how: {
    kicker: 'Com funciona',
    title: 'Tres passos, sense manual',
    steps: [
      { title: 'Tria què fas', text: 'Un gest amb els dits, una combinació de tecles o un botó del ratolí. El graves només fent-lo.' },
      { title: 'Tria què passa', text: 'Col·locar la finestra, obrir una app, enviar una drecera, apujar el volum… Ho tries d’una llista.' },
      { title: 'I ja està', text: 'Funciona a totes les apps, o només a les que triïs. Els canvis es desen sols.' },
    ],
  },

  actions: {
    kicker: 'El que MacControl sap fer',
    title: 'Un gest, molts trucs',
    groups: [
      { title: 'Finestres', items: ['Meitats, terços i quarts', 'Maximitzar i centrar', 'Moure a la pantalla següent', 'Restaurar la mida anterior', 'Minimitzar, tancar, pantalla completa'] },
      { title: 'Sistema', items: ['Mission Control i App Exposé', 'Mostrar l’escriptori i Launchpad', 'Bloquejar la pantalla i captura', 'Apujar, abaixar i silenciar el volum', 'Clic esquerre, dret i central'] },
      { title: 'A la teva mida', items: ['Enviar qualsevol drecera de teclat', 'Obrir una app', 'Obrir un web o un fitxer', 'Escriure un text', 'Executar una ordre de terminal'] },
    ],
  },

  gallery: {
    kicker: 'Captures',
    title: 'Fes-hi una ullada',
    open: 'Obrir la captura: {caption}',
    close: 'Tancar',
    prev: 'Captura anterior',
    next: 'Captura següent',
    items: {
      home: 'Inici: tot d’un cop d’ull',
      trackpad: 'Gestos del trackpad, amb un provador en directe',
      keyboard: 'Dreceres de teclat',
      mouse: 'Botons del ratolí i roda',
      snapping: 'Ajust de finestres i les seves opcions',
      settings: 'Ajustos, permisos i estat',
    },
  },

  install: {
    kicker: 'Instal·lar',
    title: 'En marxa en dos minuts',
    steps: [
      'Descarrega el fitxer <strong>.dmg</strong> de l’última versió a GitHub i obre’l.',
      'Arrossega <strong>MacControl</strong> a la carpeta <strong>Aplicacions</strong>.',
      'La primera vegada, macOS dirà que no pot comprovar l’app. Ves a <strong>Configuració del Sistema → Privadesa i seguretat</strong> i fes clic a <strong>Obre igualment</strong>.',
      'Permet MacControl a <strong>Privadesa i seguretat → Accessibilitat</strong> (i a <strong>Monitoratge de l’entrada</strong> si t’ho demana). Ho necessita per llegir les dreceres i moure les finestres.',
    ],
    note: 'Requereix macOS 12 o posterior en un Mac amb xip d’Apple.',
    cta: 'Descarregar l’última versió',
  },

  faq: {
    kicker: 'Preguntes',
    title: 'Preguntes freqüents',
    items: [
      { q: 'MacControl és gratuïta?', a: 'Sí. És gratuïta de descarregar i d’utilitzar, sense comptes, sense anuncis i sense compres dins l’app.' },
      { q: 'Per què necessita el permís d’Accessibilitat?', a: 'Per detectar les teves dreceres i els botons del ratolí a qualsevol app, i per moure i canviar la mida de les finestres d’altres apps. macOS demana aquest permís a qualsevol app que faci aquestes coses.' },
      { q: 'Registra el que escric?', a: 'No. MacControl només compara cada tecla amb les teves regles i deixa passar la resta. No desa ni envia res, i el codi és a GitHub perquè qualsevol el pugui llegir.' },
      { q: 'macOS diu que no pot obrir l’app. Què faig?', a: 'MacControl encara no està notaritzada per Apple. Ves a <strong>Configuració del Sistema → Privadesa i seguretat</strong> i fes clic a <strong>Obre igualment</strong> al costat del missatge sobre MacControl. Només cal fer-ho una vegada.' },
      { q: 'En quins Macs funciona?', a: 'En Macs amb xip d’Apple (M1 o posterior) amb macOS 12 Monterey o posterior. Els gestos necessiten el trackpad del portàtil o un Magic Trackpad.' },
      { q: 'Què passa quan tanco la finestra?', a: 'MacControl continua funcionant des de la icona del barret de copa a la barra de menús, i la icona del Dock desapareix. Per tancar-la del tot, tria <strong>Salir de MacControl</strong> (Sortir) al menú d’aquesta icona.' },
      { q: 'En quin idioma és l’app?', a: 'En català, castellà i anglès: fa servir l’idioma del teu Mac i el pots canviar a la Configuració.' },
    ],
  },

  cta: {
    title: 'A punt per a una mica de màgia?',
    text: 'Descarrega MacControl i fes que el teu Mac treballi com tu.',
    button: 'Descarregar per a Mac',
  },

  footer: {
    tagline: 'Gestos del trackpad, dreceres de teclat, botons del ratolí i ajust de finestres per a macOS.',
    madeBy: 'Dissenyada i desenvolupada amb {heart} a Arenys de Munt per {author}.',
    disclaimer: 'Mac, macOS, Magic Trackpad i Mission Control són marques d’Apple Inc. MacControl és una app independent, sense relació amb Apple ni avalada per ella.',
    links: 'Enllaços',
    source: 'Codi font',
    releases: 'Versions',
    cookies: 'Preferències de galetes',
    languages: 'Idiomes',
  },

  consent: {
    title: 'Galetes',
    text: 'Fem servir galetes de Google Analytics per comptar les visites i veure quines parts d’aquesta pàgina són útils. Només s’instal·len si les acceptes.',
    accept: 'Acceptar',
    reject: 'Rebutjar',
  },

  suggest: {
    text: 'Aquesta pàgina també és en català.',
    cta: 'Veure en català',
    dismiss: 'Tancar',
  },

  notFound: {
    title: 'No hi ha res dins d’aquest barret',
    text: 'La pàgina que busques no és aquí. Potser s’ha mogut o no ha existit mai.',
    back: 'Tornar a l’inici',
    metaTitle: 'Pàgina no trobada · MacControl',
  },
};

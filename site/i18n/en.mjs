export default {
  lang: 'en',
  locale: 'en_GB',
  name: 'English',

  meta: {
    title: 'MacControl: trackpad gestures, shortcuts and window snapping for Mac',
    description:
      'Free Mac app: give your trackpad gestures, keyboard shortcuts and mouse buttons new jobs, and snap windows into place by dragging them to the edges of the screen.',
    ogTitle: 'MacControl: your Mac, your way',
    ogAlt: 'MacControl: a black top hat on a cyan tile next to the app’s home screen',
  },

  skip: 'Skip to content',
  nav: {
    label: 'Main',
    features: 'Features',
    demo: 'Try it',
    gallery: 'Screenshots',
    install: 'Install',
    faq: 'FAQ',
    download: 'Download',
    menu: 'Menu',
    language: 'Language',
  },

  hero: {
    eyebrow: 'Free for macOS · v{version}',
    title: 'Your Mac, <em>your&nbsp;way</em>',
    lead:
      'Give your trackpad gestures, keyboard shortcuts and mouse buttons new jobs, and snap windows into place just by dragging them to the edges of the screen. One small app that lives in your menu bar.',
    ctaPrimary: 'Download for Mac',
    ctaSecondary: 'View on GitHub',
    points: ['Free, no account', 'Nothing leaves your Mac', 'Works in every app'],
    imageAlt: 'MacControl’s home screen: shortcuts to trackpad gestures, keyboard shortcuts, mouse buttons and window snapping',
    chipA: { keys: '⌃⌥←', text: 'Left half' },
    chipB: { keys: '3 fingers', text: 'Middle click' },
  },

  ticker: [
    '3-finger tap → Middle click',
    '⌃⌥← → Left half',
    'Side button → Back',
    '4-finger tap → Launchpad',
    'Drag to the top → Maximise',
    'TipTap left → ⌘[',
    '⌥ + middle click → Mission Control',
    'Corner tap → Show desktop',
  ],

  features: {
    kicker: 'Four tricks up its sleeve',
    title: 'Everything you touch, a little smarter',
    intro: 'Each rule is one sentence: “when I do this, MacControl does that”. Make it work everywhere or only in the apps you choose.',
    items: [
      {
        icon: 'trackpad', tone: 'violet', title: 'Trackpad gestures',
        text: 'Taps, double taps and clicks with 2 to 5 fingers, 3, 4 and 5-finger swipes, one-finger corner taps, swipes in from the edge and TipTap.',
        example: { keys: '3 fingers', text: 'Middle click' },
      },
      {
        icon: 'keyboard', tone: 'cyan', title: 'Keyboard shortcuts',
        text: 'Global shortcuts that do whatever you want: place the window, open an app or a link, type a snippet of text or run a command.',
        example: { keys: '⌃⌥↩', text: 'Maximise' },
      },
      {
        icon: 'mouse', tone: 'pink', title: 'Mouse buttons',
        text: 'Side buttons, the middle button and the scroll wheel combined with modifier keys. Back and forward in your browser, out of the box.',
        example: { keys: 'Button 4', text: 'Back' },
      },
      {
        icon: 'window', tone: 'mint', title: 'Window snapping',
        text: 'Drag a window to the left or right edge for a half, to the top to maximise it and to a corner for a quarter. A preview shows where it will land.',
        example: { keys: 'Drag ←', text: 'Left half' },
      },
    ],
  },

  demo: {
    kicker: 'Try it',
    title: 'Drag the window to an edge',
    intro: 'This is how snapping feels. Drag the window by its title bar to an edge or a corner of this little screen and let go. Or use the buttons and shortcuts below.',
    windowTitle: 'Notes',
    windowText: 'Drag me by the title bar',
    hint: 'Drag to an edge or a corner',
    keys: 'Keyboard: <kbd>⌃</kbd><kbd>⌥</kbd><kbd>←</kbd> left · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>→</kbd> right · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>↩</kbd> maximise · <kbd>⌃</kbd><kbd>⌥</kbd><kbd>C</kbd> centre',
    buttons: 'Place the window',
    positions: {
      left: 'Left half', right: 'Right half', max: 'Maximise', center: 'Centre',
      tl: 'Top-left quarter', tr: 'Top-right quarter', bl: 'Bottom-left quarter', br: 'Bottom-right quarter',
    },
    moved: 'Window moved: {pos}',
  },

  how: {
    kicker: 'How it works',
    title: 'Three steps, no manual',
    steps: [
      { title: 'Choose what you do', text: 'A gesture with your fingers, a key combination or a mouse button. Record it by just doing it.' },
      { title: 'Choose what happens', text: 'Place the window, open an app, send a shortcut, turn the volume up… Pick from the list.' },
      { title: 'And that’s it', text: 'It works in every app, or only in the ones you choose. Changes save themselves.' },
    ],
  },

  actions: {
    kicker: 'What MacControl can do',
    title: 'One trigger, many tricks',
    groups: [
      { title: 'Windows', items: ['Halves, thirds and quarters', 'Maximise and centre', 'Move to the next display', 'Restore the previous size', 'Minimise, close, full screen'] },
      { title: 'System', items: ['Mission Control and App Exposé', 'Show desktop and Launchpad', 'Lock screen and screenshot', 'Volume up, down and mute', 'Left, right and middle click'] },
      { title: 'Your own', items: ['Send any keyboard shortcut', 'Open an app', 'Open a website or a file', 'Type a snippet of text', 'Run a shell command'] },
    ],
  },

  gallery: {
    kicker: 'Screenshots',
    title: 'Take a look around',
    open: 'Open screenshot: {caption}',
    close: 'Close',
    prev: 'Previous screenshot',
    next: 'Next screenshot',
    items: {
      home: 'Home: everything at a glance',
      trackpad: 'Trackpad gestures, with a live tester',
      keyboard: 'Keyboard shortcuts',
      mouse: 'Mouse buttons and scroll wheel',
      snapping: 'Window snapping and its settings',
      settings: 'Settings, permissions and status',
    },
  },

  install: {
    kicker: 'Install',
    title: 'Up and running in two minutes',
    steps: [
      'Download the <strong>.dmg</strong> file from the latest release on GitHub and open it.',
      'Drag <strong>MacControl</strong> into your <strong>Applications</strong> folder.',
      'The first time, macOS will say it can’t check the app. Go to <strong>System Settings → Privacy &amp; Security</strong> and click <strong>Open Anyway</strong>.',
      'Allow MacControl in <strong>Privacy &amp; Security → Accessibility</strong> (and <strong>Input Monitoring</strong> if it asks). It needs this to read shortcuts and move windows.',
    ],
    note: 'Requires macOS 12 or later on a Mac with Apple silicon.',
    cta: 'Get the latest release',
  },

  faq: {
    kicker: 'FAQ',
    title: 'Frequently asked questions',
    items: [
      { q: 'Is MacControl free?', a: 'Yes. It’s free to download and use, with no account, no ads and no in-app purchases.' },
      { q: 'Why does it need Accessibility permission?', a: 'To notice your shortcuts and mouse buttons in any app, and to move and resize other apps’ windows. macOS asks for this permission for every app that does these things.' },
      { q: 'Does it record what I type?', a: 'No. MacControl only checks each key press against your rules and lets the rest through. Nothing is stored or sent anywhere, and the source code is on GitHub for anyone to read.' },
      { q: 'macOS says the app can’t be opened. What do I do?', a: 'MacControl isn’t notarised by Apple yet. Go to <strong>System Settings → Privacy &amp; Security</strong> and click <strong>Open Anyway</strong> next to the message about MacControl. You only need to do this once.' },
      { q: 'Which Macs does it run on?', a: 'Macs with Apple silicon (M1 or later) running macOS 12 Monterey or later. Gestures need a built-in trackpad or a Magic Trackpad.' },
      { q: 'What happens when I close the window?', a: 'MacControl keeps working from its top-hat icon in the menu bar, and its Dock icon goes away. To quit it completely, choose <strong>Salir de MacControl</strong> (Quit) from that icon’s menu.' },
      { q: 'Is the app in English?', a: 'Yes. The interface is in English, Spanish and Catalan: it follows your Mac’s language and you can change it in Settings.' },
    ],
  },

  cta: {
    title: 'Ready for a little magic?',
    text: 'Download MacControl and make your Mac work the way you do.',
    button: 'Download for Mac',
  },

  footer: {
    tagline: 'Trackpad gestures, keyboard shortcuts, mouse buttons and window snapping for macOS.',
    madeBy: 'Designed and developed with {heart} in Arenys de Munt by {author}.',
    disclaimer: 'Mac, macOS, Magic Trackpad and Mission Control are trademarks of Apple Inc. MacControl is an independent app and is not affiliated with or endorsed by Apple.',
    links: 'Links',
    source: 'Source code',
    releases: 'Releases',
    cookies: 'Cookie settings',
    languages: 'Languages',
  },

  consent: {
    title: 'Cookies',
    text: 'We use Google Analytics cookies to count visits and see which parts of this page are useful. They’re only set if you accept.',
    accept: 'Accept',
    reject: 'Reject',
  },

  suggest: {
    text: 'This page is also available in English.',
    cta: 'Switch to English',
    dismiss: 'Dismiss',
  },

  notFound: {
    title: 'Nothing up this sleeve',
    text: 'The page you’re looking for isn’t here. It may have moved or never existed.',
    back: 'Back to the home page',
    metaTitle: 'Page not found · MacControl',
  },
};

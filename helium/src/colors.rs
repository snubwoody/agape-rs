use helium_core::Color;

/// Contains all the colors from [tailwindcss](https://tailwindcss.com/docs/customizing-colors).
pub mod tailwind_colors {
    use super::*;

    pub const SLATE50: Color = Color::Hex("#f8fafc");
    pub const SLATE100: Color = Color::Hex("#f1f5f9");
    pub const SLATE200: Color = Color::Hex("#e2e8f0");
    pub const SLATE300: Color = Color::Hex("#cbd5e1");
    pub const SLATE400: Color = Color::Hex("#94a3b8");
    pub const SLATE500: Color = Color::Hex("#64748b");
    pub const SLATE600: Color = Color::Hex("#475569");
    pub const SLATE700: Color = Color::Hex("#334155");
    pub const SLATE800: Color = Color::Hex("#1e293b");
    pub const SLATE900: Color = Color::Hex("#0f172a");
    pub const SLATE950: Color = Color::Hex("#020617");

    pub const GRAY50: Color = Color::Hex("#f9fafb");
    pub const GRAY100: Color = Color::Hex("#f3f4f6");
    pub const GRAY200: Color = Color::Hex("#e5e7eb");
    pub const GRAY300: Color = Color::Hex("#d1d5db");
    pub const GRAY400: Color = Color::Hex("#9ca3af");
    pub const GRAY500: Color = Color::Hex("#6b7280");
    pub const GRAY600: Color = Color::Hex("#4b5563");
    pub const GRAY700: Color = Color::Hex("#374151");
    pub const GRAY800: Color = Color::Hex("#1f2937");
    pub const GRAY900: Color = Color::Hex("#111827");
    pub const GRAY950: Color = Color::Hex("#030712");

    pub const ZINC50: Color = Color::Hex("#fafafa");
    pub const ZINC100: Color = Color::Hex("'#f4f4f5");
    pub const ZINC200: Color = Color::Hex("'#e4e4e7");
    pub const ZINC300: Color = Color::Hex("'#d4d4d8");
    pub const ZINC400: Color = Color::Hex("'#a1a1aa");
    pub const ZINC500: Color = Color::Hex("'#71717a");
    pub const ZINC600: Color = Color::Hex("'#52525b");
    pub const ZINC700: Color = Color::Hex("'#3f3f46");
    pub const ZINC800: Color = Color::Hex("'#27272a");
    pub const ZINC900: Color = Color::Hex("'#18181b");
    pub const ZINC950: Color = Color::Hex("'#09090b");

    pub const NEUTRAL50: Color = Color::Hex("#fafafa");
    pub const NEUTRAL100: Color = Color::Hex("#f5f5f5");
    pub const NEUTRAL200: Color = Color::Hex("#e5e5e5");
    pub const NEUTRAL300: Color = Color::Hex("#d4d4d4");
    pub const NEUTRAL400: Color = Color::Hex("#a3a3a3");
    pub const NEUTRAL500: Color = Color::Hex("#737373");
    pub const NEUTRAL600: Color = Color::Hex("#525252");
    pub const NEUTRAL700: Color = Color::Hex("#404040");
    pub const NEUTRAL800: Color = Color::Hex("#262626");
    pub const NEUTRAL900: Color = Color::Hex("#171717");
    pub const NEUTRAL950: Color = Color::Hex("#0a0a0a");

    pub const STONE50: Color = Color::Hex("#fafaf9");
    pub const STONE100: Color = Color::Hex("#f5f5f4");
    pub const STONE200: Color = Color::Hex("#e7e5e4");
    pub const STONE300: Color = Color::Hex("#d6d3d1");
    pub const STONE400: Color = Color::Hex("#a8a29e");
    pub const STONE500: Color = Color::Hex("#78716c");
    pub const STONE600: Color = Color::Hex("#57534e");
    pub const STONE700: Color = Color::Hex("#44403c");
    pub const STONE800: Color = Color::Hex("#292524");
    pub const STONE900: Color = Color::Hex("#1c1917");
    pub const STONE950: Color = Color::Hex("#0c0a09");

    pub const RED50: Color = Color::Hex("#fef2f2");
    pub const RED100: Color = Color::Hex("#fee2e2");
    pub const RED200: Color = Color::Hex("#fecaca");
    pub const RED300: Color = Color::Hex("#fca5a5");
    pub const RED400: Color = Color::Hex("#f87171");
    pub const RED500: Color = Color::Hex("#ef4444");
    pub const RED600: Color = Color::Hex("#dc2626");
    pub const RED700: Color = Color::Hex("#b91c1c");
    pub const RED800: Color = Color::Hex("#991b1b");
    pub const RED900: Color = Color::Hex("#7f1d1d");
    pub const RED950: Color = Color::Hex("#450a0a");

    pub const ORANGE50: Color = Color::Hex("#fff7ed");
    pub const ORANGE100: Color = Color::Hex("#ffedd5");
    pub const ORANGE200: Color = Color::Hex("#fed7aa");
    pub const ORANGE300: Color = Color::Hex("#fdba74");
    pub const ORANGE400: Color = Color::Hex("#fb923c");
    pub const ORANGE500: Color = Color::Hex("#f97316");
    pub const ORANGE600: Color = Color::Hex("#ea580c");
    pub const ORANGE700: Color = Color::Hex("#c2410c");
    pub const ORANGE800: Color = Color::Hex("#9a3412");
    pub const ORANGE900: Color = Color::Hex("#7c2d12");
    pub const ORANGE950: Color = Color::Hex("#431407");

    // Amber
    pub const AMBER50: Color = Color::Hex("#fffbeb");
    pub const AMBER100: Color = Color::Hex("#fef3c7");
    pub const AMBER200: Color = Color::Hex("#fde68a");
    pub const AMBER300: Color = Color::Hex("#fcd34d");
    pub const AMBER400: Color = Color::Hex("#fbbf24");
    pub const AMBER500: Color = Color::Hex("#f59e0b");
    pub const AMBER600: Color = Color::Hex("#d97706");
    pub const AMBER700: Color = Color::Hex("#b45309");
    pub const AMBER800: Color = Color::Hex("#92400e");
    pub const AMBER900: Color = Color::Hex("#78350f");
    pub const AMBER950: Color = Color::Hex("#451a03");

    // Yellow
    pub const YELLOW50: Color = Color::Hex("#fefce8");
    pub const YELLOW100: Color = Color::Hex("#fef9c3");
    pub const YELLOW200: Color = Color::Hex("#fef08a");
    pub const YELLOW300: Color = Color::Hex("#fde047");
    pub const YELLOW400: Color = Color::Hex("#facc15");
    pub const YELLOW500: Color = Color::Hex("#eab308");
    pub const YELLOW600: Color = Color::Hex("#ca8a04");
    pub const YELLOW700: Color = Color::Hex("#a16207");
    pub const YELLOW800: Color = Color::Hex("#854d0e");
    pub const YELLOW900: Color = Color::Hex("#713f12");
    pub const YELLOW950: Color = Color::Hex("#422006");

    // Lime
    pub const LIME50: Color = Color::Hex("#f7fee7");
    pub const LIME100: Color = Color::Hex("#ecfccb");
    pub const LIME200: Color = Color::Hex("#d9f99d");
    pub const LIME300: Color = Color::Hex("#bef264");
    pub const LIME400: Color = Color::Hex("#a3e635");
    pub const LIME500: Color = Color::Hex("#84cc16");
    pub const LIME600: Color = Color::Hex("#65a30d");
    pub const LIME700: Color = Color::Hex("#4d7c0f");
    pub const LIME800: Color = Color::Hex("#3f6212");
    pub const LIME900: Color = Color::Hex("#365314");
    pub const LIME950: Color = Color::Hex("#1a2e05");

    // Green
    pub const GREEN50: Color = Color::Hex("#f0fdf4");
    pub const GREEN100: Color = Color::Hex("#dcfce7");
    pub const GREEN200: Color = Color::Hex("#bbf7d0");
    pub const GREEN300: Color = Color::Hex("#86efac");
    pub const GREEN400: Color = Color::Hex("#4ade80");
    pub const GREEN500: Color = Color::Hex("#22c55e");
    pub const GREEN600: Color = Color::Hex("#16a34a");
    pub const GREEN700: Color = Color::Hex("#15803d");
    pub const GREEN800: Color = Color::Hex("#166534");
    pub const GREEN900: Color = Color::Hex("#14532d");
    pub const GREEN950: Color = Color::Hex("#052e16");

    // Emerald
    pub const EMERALD50: Color = Color::Hex("#ecfdf5");
    pub const EMERALD100: Color = Color::Hex("#d1fae5");
    pub const EMERALD200: Color = Color::Hex("#a7f3d0");
    pub const EMERALD300: Color = Color::Hex("#6ee7b7");
    pub const EMERALD400: Color = Color::Hex("#34d399");
    pub const EMERALD500: Color = Color::Hex("#10b981");
    pub const EMERALD600: Color = Color::Hex("#059669");
    pub const EMERALD700: Color = Color::Hex("#047857");
    pub const EMERALD800: Color = Color::Hex("#065f46");
    pub const EMERALD900: Color = Color::Hex("#064e3b");
    pub const EMERALD950: Color = Color::Hex("#022c22");

    // Teal
    pub const TEAL50: Color = Color::Hex("#f0fdfa");
    pub const TEAL100: Color = Color::Hex("#ccfbf1");
    pub const TEAL200: Color = Color::Hex("#99f6e4");
    pub const TEAL300: Color = Color::Hex("#5eead4");
    pub const TEAL400: Color = Color::Hex("#2dd4bf");
    pub const TEAL500: Color = Color::Hex("#14b8a6");
    pub const TEAL600: Color = Color::Hex("#0d9488");
    pub const TEAL700: Color = Color::Hex("#0f766e");
    pub const TEAL800: Color = Color::Hex("#115e59");
    pub const TEAL900: Color = Color::Hex("#134e4a");
    pub const TEAL950: Color = Color::Hex("#042f2e");

    // Cyan
    pub const CYAN50: Color = Color::Hex("#ecfeff");
    pub const CYAN100: Color = Color::Hex("#cffafe");
    pub const CYAN200: Color = Color::Hex("#a5f3fc");
    pub const CYAN300: Color = Color::Hex("#67e8f9");
    pub const CYAN400: Color = Color::Hex("#22d3ee");
    pub const CYAN500: Color = Color::Hex("#06b6d4");
    pub const CYAN600: Color = Color::Hex("#0891b2");
    pub const CYAN700: Color = Color::Hex("#0e7490");
    pub const CYAN800: Color = Color::Hex("#155e75");
    pub const CYAN900: Color = Color::Hex("#164e63");
    pub const CYAN950: Color = Color::Hex("#083344");

    // Sky
    pub const SKY50: Color = Color::Hex("#f0f9ff");
    pub const SKY100: Color = Color::Hex("#e0f2fe");
    pub const SKY200: Color = Color::Hex("#bae6fd");
    pub const SKY300: Color = Color::Hex("#7dd3fc");
    pub const SKY400: Color = Color::Hex("#38bdf8");
    pub const SKY500: Color = Color::Hex("#0ea5e9");
    pub const SKY600: Color = Color::Hex("#0284c7");
    pub const SKY700: Color = Color::Hex("#0369a1");
    pub const SKY800: Color = Color::Hex("#075985");
    pub const SKY900: Color = Color::Hex("#0c4a6e");
    pub const SKY950: Color = Color::Hex("#082f49");

    // Blue
    pub const BLUE50: Color = Color::Hex("#eff6ff");
    pub const BLUE100: Color = Color::Hex("#dbeafe");
    pub const BLUE200: Color = Color::Hex("#bfdbfe");
    pub const BLUE300: Color = Color::Hex("#93c5fd");
    pub const BLUE400: Color = Color::Hex("#60a5fa");
    pub const BLUE500: Color = Color::Hex("#3b82f6");
    pub const BLUE600: Color = Color::Hex("#2563eb");
    pub const BLUE700: Color = Color::Hex("#1d4ed8");
    pub const BLUE800: Color = Color::Hex("#1e40af");
    pub const BLUE900: Color = Color::Hex("#1e3a8a");
    pub const BLUE950: Color = Color::Hex("#172554");

    // Indigo
    pub const INDIGO50: Color = Color::Hex("#eef2ff");
    pub const INDIGO100: Color = Color::Hex("#e0e7ff");
    pub const INDIGO200: Color = Color::Hex("#c7d2fe");
    pub const INDIGO300: Color = Color::Hex("#a5b4fc");
    pub const INDIGO400: Color = Color::Hex("#818cf8");
    pub const INDIGO500: Color = Color::Hex("#6366f1");
    pub const INDIGO600: Color = Color::Hex("#4f46e5");
    pub const INDIGO700: Color = Color::Hex("#4338ca");
    pub const INDIGO800: Color = Color::Hex("#3730a3");
    pub const INDIGO900: Color = Color::Hex("#312e81");
    pub const INDIGO950: Color = Color::Hex("#1e1b4b");

    // Violet
    pub const VIOLET50: Color = Color::Hex("#f5f3ff");
    pub const VIOLET100: Color = Color::Hex("#ede9fe");
    pub const VIOLET200: Color = Color::Hex("#ddd6fe");
    pub const VIOLET300: Color = Color::Hex("#c4b5fd");
    pub const VIOLET400: Color = Color::Hex("#a78bfa");
    pub const VIOLET500: Color = Color::Hex("#8b5cf6");
    pub const VIOLET600: Color = Color::Hex("#7c3aed");
    pub const VIOLET700: Color = Color::Hex("#6d28d9");
    pub const VIOLET800: Color = Color::Hex("#5b21b6");
    pub const VIOLET900: Color = Color::Hex("#4c1d95");
    pub const VIOLET950: Color = Color::Hex("#2e1065");

    // Purple
    pub const PURPLE50: Color = Color::Hex("#faf5ff");
    pub const PURPLE100: Color = Color::Hex("#f3e8ff");
    pub const PURPLE200: Color = Color::Hex("#e9d5ff");
    pub const PURPLE300: Color = Color::Hex("#d8b4fe");
    pub const PURPLE400: Color = Color::Hex("#c084fc");
    pub const PURPLE500: Color = Color::Hex("#a855f7");
    pub const PURPLE600: Color = Color::Hex("#9333ea");
    pub const PURPLE700: Color = Color::Hex("#7e22ce");
    pub const PURPLE800: Color = Color::Hex("#6b21a8");
    pub const PURPLE900: Color = Color::Hex("#581c87");
    pub const PURPLE950: Color = Color::Hex("#3b0764");

    // Fuchsia
    pub const FUCHSIA50: Color = Color::Hex("#fdf4ff");
    pub const FUCHSIA100: Color = Color::Hex("#fae8ff");
    pub const FUCHSIA200: Color = Color::Hex("#f5d0fe");
    pub const FUCHSIA300: Color = Color::Hex("#f0abfc");
    pub const FUCHSIA400: Color = Color::Hex("#e879f9");
    pub const FUCHSIA500: Color = Color::Hex("#d946ef");
    pub const FUCHSIA600: Color = Color::Hex("#c026d3");
    pub const FUCHSIA700: Color = Color::Hex("#a21caf");
    pub const FUCHSIA800: Color = Color::Hex("#86198f");
    pub const FUCHSIA900: Color = Color::Hex("#701a75");
    pub const FUCHSIA950: Color = Color::Hex("#4a044e");

    // Pink
    pub const PINK50: Color = Color::Hex("#fdf2f8");
    pub const PINK100: Color = Color::Hex("#fce7f3");
    pub const PINK200: Color = Color::Hex("#fbcfe8");
    pub const PINK300: Color = Color::Hex("#f9a8d4");
    pub const PINK400: Color = Color::Hex("#f472b6");
    pub const PINK500: Color = Color::Hex("#ec4899");
    pub const PINK600: Color = Color::Hex("#db2777");
    pub const PINK700: Color = Color::Hex("#be185d");
    pub const PINK800: Color = Color::Hex("#9d174d");
    pub const PINK900: Color = Color::Hex("#831843");
    pub const PINK950: Color = Color::Hex("#500724");

    // Rose
    pub const ROSE50: Color = Color::Hex("#fff1f2");
    pub const ROSE100: Color = Color::Hex("#ffe4e6");
    pub const ROSE200: Color = Color::Hex("#fecdd3");
    pub const ROSE300: Color = Color::Hex("#fda4af");
    pub const ROSE400: Color = Color::Hex("#fb7185");
    pub const ROSE500: Color = Color::Hex("#f43f5e");
    pub const ROSE600: Color = Color::Hex("#e11d48");
    pub const ROSE700: Color = Color::Hex("#be123c");
    pub const ROSE800: Color = Color::Hex("#9f1239");
    pub const ROSE900: Color = Color::Hex("#881337");
    pub const ROSE950: Color = Color::Hex("#4c0519");
}

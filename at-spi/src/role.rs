use crate::Object;
use serde::Deserialize;
use zbus::zvariant::Type;

#[derive(Debug, Type, Deserialize, Hash, PartialEq)]
pub enum Role {
    Invalid,
    AcceleratorLabel,
    Alert,
    Animation,
    Arrow,
    Calendar,
    Canvas,
    CheckBox,
    CheckMenuItem,
    ColorChooser,
    ColumnHeader,
    ComboBox,
    DateEditor,
    DesktopIcon,
    DesktopFrame,
    Dial,
    Dialog,
    DirectoryPane,
    DrawingArea,
    FileChooser,
    Filler,
    FocusTraversable,
    FontChooser,
    Frame,
    GlassPane,
    HtmlContainer,
    Icon,
    Image,
    InternalFrame,
    Label,
    LayeredPane,
    List,
    ListItem,
    Menu,
    MenuBar,
    MenuItem,
    OptionPane,
    PageTab,
    PageTabList,
    Panel,
    PasswordText,
    PopupMenU,
    ProgressBar,
    PushButton,
    RadioButton,
    RadioMenuItem,
    RootPane,
    RowHeader,
    ScrollBar,
    ScrollPane,
    Separator,
    Slider,
    SpinButton,
    SplitPane,
    StatusBar,
    Table,
    TableCell,
    TableColumnHeader,
    TableRowHeader,
    TearoffMenuItem,
    Terminal,
    Text,
    ToggleButton,
    ToolBar,
    ToolTip,
    Tree,
    TreeTable,
    Unknown,
    Viewport,
    Window,
    Extended,
    Header,
    Footer,
    Paragraph,
    Ruler,
    Application,
    Autocomplete,
    Editbar,
    Embedded,
    Entry,
    Chart,
    Caption,
    DocumentFrame,
    Heading,
    Page,
    Section,
    RedundantObject,
    Form,
    Link,
    InputMethodWindow,
    TableRow,
    TreeItem,
    DocumentSpreadsheet,
    DocumentPresentation,
    DocumentText,
    DocumentWeb,
    DocumentEmail,
    Comment,
    ListBox,
    Grouping,
    ImageMap,
    Notification,
    InfoBar,
    LevelBar,
    TitleBar,
    BlockQuote,
    Audio,
    Video,
    Definition,
    Article,
    Landmark,
    Log,
    Marquee,
    Math,
    Rating,
    Timer,
    Static,
    MathFraction,
    MathRoot,
    Subscript,
    Superscript,
    DescriptionList,
    DescriptionTerm,
    DescriptionValue,
    Footnote,
}

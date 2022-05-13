#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventKind(&'static str);

#[allow(non_upper_case_globals)]
impl EventKind {
    pub const Abort: &'static str = "abort";
    pub const Autocomplete: &'static str = "autocomplete";
    pub const AutocompleteError: &'static str = "autocompleteerror";
    pub const Blur: &'static str = "blur";
    pub const Cancel: &'static str = "cancel";
    pub const CanPlay: &'static str = "canplay";
    pub const CanPlayThrough: &'static str = "canplaythrough";
    pub const Change: &'static str = "change";
    pub const Click: &'static str = "click";
    pub const Close: &'static str = "close";
    pub const ContextMenu: &'static str = "contextmenu";
    pub const CueChange: &'static str = "cuechange";
    pub const DblClick: &'static str = "dblclick";
    pub const Drag: &'static str = "drag";
    pub const DragEnd: &'static str = "dragend";
    pub const DragEnter: &'static str = "dragenter";
    pub const DragExit: &'static str = "dragexit";
    pub const DragLeave: &'static str = "dragleave";
    pub const DragOver: &'static str = "dragover";
    pub const DragStart: &'static str = "dragstart";
    pub const Drop: &'static str = "drop";
    pub const DurationChange: &'static str = "durationchange";
    pub const Emptied: &'static str = "emptied";
    pub const Ended: &'static str = "ended";
    pub const Error: &'static str = "error";
    pub const Focus: &'static str = "focus";
    pub const Input: &'static str = "input";
    pub const Invalid: &'static str = "invalid";
    pub const KeyDown: &'static str = "keydown";
    pub const Keypress: &'static str = "keypress";
    pub const KeyUp: &'static str = "keyup";
    pub const Load: &'static str = "load";
    pub const LoadedData: &'static str = "loadeddata";
    pub const LoadedMetadata: &'static str = "loadedmetadata";
    pub const LoadStart: &'static str = "loadstart";
    pub const MouseDown: &'static str = "mousedown";
    pub const MouseEnter: &'static str = "mouseenter";
    pub const MouseLeave: &'static str = "mouseleave";
    pub const MouseMove: &'static str = "mousemove";
    pub const MouseOut: &'static str = "mouseout";
    pub const Mouseover: &'static str = "mouseover";
    pub const MouseUp: &'static str = "mouseup";
    pub const MouseWheel: &'static str = "mousewheel";
    pub const Pause: &'static str = "pause";
    pub const Play: &'static str = "play";
    pub const Playing: &'static str = "playing";
    pub const Progress: &'static str = "progress";
    pub const RateChange: &'static str = "ratechange";
    pub const Reset: &'static str = "reset";
    pub const Resize: &'static str = "resize";
    pub const Scroll: &'static str = "scroll";
    pub const Seeked: &'static str = "seeked";
    pub const Seeking: &'static str = "seeking";
    pub const Select: &'static str = "select";
    pub const Show: &'static str = "show";
    pub const Sort: &'static str = "sort";
    pub const Stalled: &'static str = "stalled";
    pub const Submit: &'static str = "submit";
    pub const Suspend: &'static str = "suspend";
    pub const TimeUpdate: &'static str = "timeupdate";
    pub const Toggle: &'static str = "toggle";
    pub const VolumeChange: &'static str = "volumechange";
    pub const Waiting: &'static str = "waiting";

    pub fn new(kind: &'static str) -> Self {
        Self(kind)
    }
}

impl AsRef<str> for EventKind {
    fn as_ref(&self) -> &str {
        self.0
    }
}

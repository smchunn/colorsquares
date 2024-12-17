fn main() {
    // ANSI escape codes
    let esc = "\x1b";

    // Foreground colors
    let rd_f = format!("{esc}[31m");
    let gr_f = format!("{}[32m", esc);
    let yl_f = format!("{}[33m", esc);
    let bl_f = format!("{}[34m", esc);
    let mg_f = format!("{}[35m", esc);
    let cy_f = format!("{}[36m", esc);

    // Bright foreground colors
    let rd_fb = format!("{}[91m", esc);
    let gr_fb = format!("{}[92m", esc);
    let yl_fb = format!("{}[93m", esc);
    let bl_fb = format!("{}[94m", esc);
    let mg_fb = format!("{}[95m", esc);
    let cy_fb = format!("{}[96m", esc);

    // Reset
    let reset = format!("{}[0m", esc);

    // Output colord_ text
    println!();
    println!(" \
{rd_f}▀ █ {rd_fb}█ ▀{reset}   \
{gr_f}▀ █ {gr_fb}█ ▀{reset}   \
{yl_f}▀ █ {yl_fb}█ ▀{reset}   \
{bl_f}▀ █ {bl_fb}█ ▀{reset}   \
{mg_f}▀ █ {mg_fb}█ ▀{reset}   \
{cy_f}▀ █ {cy_fb}█ ▀{reset}");

    println!(" \
{rd_f}██  {rd_fb} ██{reset}   \
{gr_f}██  {gr_fb} ██{reset}   \
{yl_f}██  {yl_fb} ██{reset}   \
{bl_f}██  {bl_fb} ██{reset}   \
{mg_f}██  {mg_fb} ██{reset}   \
{cy_f}██  {cy_fb} ██{reset}");

    println!(" \
{rd_f}▄ █ {rd_fb}█ ▄{reset}   \
{gr_f}▄ █ {gr_fb}█ ▄{reset}   \
{yl_f}▄ █ {yl_fb}█ ▄{reset}   \
{bl_f}▄ █ {bl_fb}█ ▄{reset}   \
{mg_f}▄ █ {mg_fb}█ ▄{reset}   \
{cy_f}▄ █ {cy_fb}█ ▄{reset}");
    println!();
}


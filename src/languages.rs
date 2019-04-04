use std::error::*;
use std::collections::*;
use onig::Regex;
use crate::Languages;


#[derive(Debug)]
struct LangExtension (String, String);


impl LangExtension {
    fn new(s0: &str, s1: &str) -> LangExtension
    {
        LangExtension(s0.to_owned(), s1.to_owned())
    }
}

pub struct Langs {
    names: HashMap<String, String>,  // map from lowercase name to language name of choice
    extensions: HashMap<String, String>  // map from lowercase name to language extension
}


impl Langs {
    pub fn new(langs: &Languages) -> Result<Langs, Box<dyn Error>> {
        let trim_cat_re = Regex::new(r"Category:")?;

        let name_map
            = langs.categorymembers
                       .iter()
                       .map(|n| trim_cat_re.replace(&n.title,""))
                       .filter(|lang| lang != "Livecode") // should be "LiveCode" but both exist
                       .map(|n| (n.to_lowercase(), n))
                       .collect();

        // Got the following long list by first running the following command in
        // RosettaCodeData/Task :
        //
        //     find . -type f -print | sed -e s=^\./[^/]*/== | sed -e 's=/[^\.]*\.=","='| sed -e 's=^="=' | sed -e 's=$="=' | sort -u > names.txt`
        //
        // and then using a rust program to remove unnecessary entries that just had the same
        // extension as the language name converted to lower case.

        let extensions 
                = [  LangExtension::new("360-Assembly", "360")
                    ,LangExtension::new("4DOS-Batch", "4dos")
                    ,LangExtension::new("6502-Assembly", "6502")
                    ,LangExtension::new("68000-Assembly", "68000")
                    ,LangExtension::new("6800-Assembly", "6800")
                    ,LangExtension::new("8051-Assembly", "8051")
                    ,LangExtension::new("8080-Assembly", "8080")
                    ,LangExtension::new("8086-Assembly", "8086")
                    ,LangExtension::new("ActionScript", "as")
                    ,LangExtension::new("ALGOL-60", "alg")
                    ,LangExtension::new("ALGOL-68", "alg")
                    ,LangExtension::new("ALGOL-W", "alg")
                    ,LangExtension::new("AmbientTalk", "ambient")
                    ,LangExtension::new("AmigaE", "amiga")
                    ,LangExtension::new("App-Inventor", "app")
                    ,LangExtension::new("Applesoft-BASIC", "applesoft")
                    ,LangExtension::new("ARM-Assembly", "arm")
                    ,LangExtension::new("Assembly", ".as")
                    ,LangExtension::new("AutoHotkey", "ahk")
                    ,LangExtension::new("Babel", "pb")
                    ,LangExtension::new("Batch-File", "bat")
                    ,LangExtension::new("BBC-BASIC", "bbc")
                    ,LangExtension::new("Befunge", "bf")
                    ,LangExtension::new("BlitzMax", "blitz")
                    ,LangExtension::new("Brainf---", "bf")
                    ,LangExtension::new("Burlesque", "blq")
                    ,LangExtension::new("Cache-ObjectScript", "cos")
                    ,LangExtension::new("C++-CLI", "cpp")
                    ,LangExtension::new("C++", "cpp")
                    ,LangExtension::new("Clipper-XBase++", "clipper")
                    ,LangExtension::new("Clojure", "clj")
                    ,LangExtension::new("CoffeeScript", "coffee")
                    ,LangExtension::new("ColdFusion", "cfm")
                    ,LangExtension::new("Commodore-BASIC", "commodore")
                    ,LangExtension::new("Common-Lisp", "lisp")
                    ,LangExtension::new("Component-Pascal", "component")
                    ,LangExtension::new("Computer-zero-Assembly", "computer")
                    ,LangExtension::new("C-sharp", "cs")
                    ,LangExtension::new("Deja-Vu", "djv")
                    ,LangExtension::new("DIV-Games-Studio", "div")
                    ,LangExtension::new("DWScript", "dw")
                    ,LangExtension::new("EDSAC-order-code", "edsac")
                    ,LangExtension::new("Eiffel", "e")
                    ,LangExtension::new("Emacs-Lisp", "l")
                    ,LangExtension::new("Erlang", "erl")
                    ,LangExtension::new("Euler-Math-Toolbox", "euler")
                    ,LangExtension::new("Forth", "fth")
                    ,LangExtension::new("Fortran", "f")
                    ,LangExtension::new("Free-Pascal", "free")
                    ,LangExtension::new("Friendly-interactive-shell", "fish")
                    ,LangExtension::new("FRISC-Assembly", "frisc")
                    ,LangExtension::new("F-Sharp", "fs")
                    ,LangExtension::new("FUZE-BASIC", "fuze")
                    ,LangExtension::new("GFA-Basic", "gfa")
                    ,LangExtension::new("Golfscript", "golf")
                    ,LangExtension::new("Haskell", "hs")
                    ,LangExtension::new("HQ9+", "hq9p")
                    ,LangExtension::new("HyperTalk", "ht")
                    ,LangExtension::new("Inform-6", "inf")
                    ,LangExtension::new("Inform-7", "inf")
                    ,LangExtension::new("Informix-4GL", "4gl")
                    ,LangExtension::new("Integer-BASIC", "integer")
                    ,LangExtension::new("Intercal", "ical")
                    ,LangExtension::new("Jacquard-Loom", "jacquard")
                    ,LangExtension::new("JAMES-II-Rule-based-Cellular-Automata", "james")
                    ,LangExtension::new("JavaFX-Script", "javafx")
                    ,LangExtension::new("JavaScript", "js")
                    ,LangExtension::new("JudoScript", "judo")
                    ,LangExtension::new("Kamailio-Script", "kamailio")
                    ,LangExtension::new("KonsolScript", "konsol")
                    ,LangExtension::new("Lambda-Prolog", "lambda")
                    ,LangExtension::new("LaTeX", "tex")
                    ,LangExtension::new("LC3-Assembly", "lc3")
                    ,LangExtension::new("Liberty-BASIC", "liberty")
                    ,LangExtension::new("LibreOffice-Basic", "libreoffice")
                    ,LangExtension::new("Lilypond", "lily")
                    ,LangExtension::new("Lisp", "l")
                    ,LangExtension::new("LiveScript", "live")
                    ,LangExtension::new("L++", "lpp")
                    ,LangExtension::new("Locomotive-Basic", "locomotive")
                    ,LangExtension::new("LOLCODE", "lol")
                    ,LangExtension::new("LotusScript", "lotus")
                    ,LangExtension::new("Mathematica", "math")
                    ,LangExtension::new("Mathprog", "math")
                    ,LangExtension::new("MATLAB", "m")
                    ,LangExtension::new("MAXScript", "max")
                    ,LangExtension::new("MIPS-Assembly", "mips")
                    ,LangExtension::new("MIRC-Scripting-Language", "mirc")
                    ,LangExtension::new("ML-I", "ml")
                    ,LangExtension::new("Modula-2", "mod2")
                    ,LangExtension::new("Modula-3", "mod3")
                    ,LangExtension::new("Moonscript", "moon")
                    ,LangExtension::new("MoonScript", "moon")
                    ,LangExtension::new("MyrtleScript", "myrtle")
                    ,LangExtension::new("MySQL", "sql")
                    ,LangExtension::new("NewtonScript", "newton")
                    ,LangExtension::new("N-t-roff", "n")
                    ,LangExtension::new("OASYS-Assembler", "oasys")
                    ,LangExtension::new("Objective-C", "m")
                    ,LangExtension::new("Object-Pascal", "object")
                    ,LangExtension::new("OoRexx", "rexx")
                    ,LangExtension::new("OpenEdge-Progress", "openedge")
                    ,LangExtension::new("Openscad", "scad")
                    ,LangExtension::new("OxygenBasic", "oxy")
                    ,LangExtension::new("Oxygene", "oxy")
                    ,LangExtension::new("PARI-GP", "pari")
                    ,LangExtension::new("PDP-11-Assembly", "pdp-11")
                    ,LangExtension::new("Perl-6", "pl6")
                    ,LangExtension::new("Perl", "pl")
                    ,LangExtension::new("PicoLisp", "l")
                    ,LangExtension::new("PlainTeX", "tex")
                    ,LangExtension::new("PL-I", "pli")
                    ,LangExtension::new("PL-M", "plm")
                    ,LangExtension::new("PL-pgSQL", "sql")
                    ,LangExtension::new("PL-SQL", "sql")
                    ,LangExtension::new("PostScript", "ps")
                    ,LangExtension::new("PowerShell", "psh")
                    ,LangExtension::new("ProDOS", "dos")
                    ,LangExtension::new("Prolog", "pro")
                    ,LangExtension::new("Pure-Data", "pure")
                    ,LangExtension::new("Python", "py")
                    ,LangExtension::new("Racket", "rkt")
                    ,LangExtension::new("RPL-2", "rpl")
                    ,LangExtension::new("RTL-2", "rtl")
                    ,LangExtension::new("Ruby", "rb")
                    ,LangExtension::new("Run-BASIC", "run")
                    ,LangExtension::new("Sather", "sa")
                    ,LangExtension::new("Scheme", "ss")
                    ,LangExtension::new("Set-LangExtensions::new", "set")
                    ,LangExtension::new("SheerPower-4GL", "4gl")
                    ,LangExtension::new("Sinclair-ZX81-BASIC", "sinclair")
                    ,LangExtension::new("SkookumScript", "skookum")
                    ,LangExtension::new("S-LangExtensions::new", "slang")
                    ,LangExtension::new("Smalltalk", "st")
                    ,LangExtension::new("Smart-BASIC", "smart")
                    ,LangExtension::new("SNOBOL4", "sno")
                    ,LangExtension::new("Snobol", "sno")
                    ,LangExtension::new("SoneKing-Assembly", "soneking")
                    ,LangExtension::new("SPARC-Assembly", "sparc")
                    ,LangExtension::new("Squirrel", "nut")
                    ,LangExtension::new("Standard-ML", "ml")
                    ,LangExtension::new("SystemVerilog", "v")
                    ,LangExtension::new("TI-83-BASIC", "ti-83")
                    ,LangExtension::new("TI-83-Hex-Assembly", "ti-83")
                    ,LangExtension::new("TI-89-BASIC", "ti-89")
                    ,LangExtension::new("TIScript", "ti")
                    ,LangExtension::new("ToffeeScript", "toffee")
                    ,LangExtension::new("TorqueScript", "torque")
                    ,LangExtension::new("Transact-SQL", "sql")
                    ,LangExtension::new("True-BASIC", "true")
                    ,LangExtension::new("TSE-SAL", "tse")
                    ,LangExtension::new("TUSCRIPT", "tu")
                    ,LangExtension::new(".", "txt")
                    ,LangExtension::new("TypeScript", "type")
                    ,LangExtension::new("UNIX-Shell", "sh")
                    ,LangExtension::new("VAX-Assembly", "vax")
                    ,LangExtension::new("VBScript", "vb")
                    ,LangExtension::new("Vedit-macro-language", "vedit")
                    ,LangExtension::new("Verilog", "v")
                    ,LangExtension::new("Vim-Script", "vim")
                    ,LangExtension::new("Visual-Basic-.NET", "visual")
                    ,LangExtension::new("Visual-Basic", "vb")
                    ,LangExtension::new("Visual-FoxPro", "visual")
                    ,LangExtension::new("Visual-Objects", "vobj")
                    ,LangExtension::new("Visual-Prolog", "pro")
                    ,LangExtension::new("Viua-VM-assembly", "viua")
                    ,LangExtension::new("Whitespace", "ws")
                    ,LangExtension::new("Wolfram-Language", "wolfram")
                    ,LangExtension::new("X86-Assembly", "x86")
                    ,LangExtension::new("XPath-2.0", "xpath")
                    ,LangExtension::new("XSLT-1.0", "xslt")
                    ,LangExtension::new("XSLT-2.0", "xslt")
                    ,LangExtension::new("Z80-Assembly", "z80")
                    ,LangExtension::new("ZX-Spectrum-Basic", "zx")
                  ];

        let special_case_extensions 
                = [  LangExtension::new("basic-or-quickbasic" , "basic")
                    ,LangExtension::new("clojure-or-clojurescript" , "clj")
                    ,LangExtension::new("Dylan.NET-or-Dylan.NET" , "dylan")
                    ,LangExtension::new("pascal-or-freepascal" , "pascal")
                    ,LangExtension::new("python-or-python-3" , "py")
                  ];

        let extensions_map 
            = extensions.iter()
                        .chain(special_case_extensions.iter())
                        .map(|le| (le.0.to_lowercase(), le.1.clone()))
                        .collect();

        Ok(Langs{names: name_map, extensions: extensions_map})
    }

    pub fn lookup(self: &Self, name: String) -> String {
        if let Some(found_name) = self.names.get(&name.to_lowercase()) {
            found_name.clone()
        } else {
            println!("LANGUAGE NOT FOUND: {}", name);
            name
        }
    }

    pub fn lookup_extension(self: &Self, name: String) -> String {
        let lc_name = name.to_lowercase();
        if let Some(found_name) = self.extensions.get(&lc_name) {
            found_name.clone()
        } else {
            lc_name
        }
    }
}


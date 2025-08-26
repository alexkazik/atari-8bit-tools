    /*
    always awailable:
    - 0080-00ff
    - 0600-06ff
    - 2000-bfff

    disabled os:
    - c000-cfff
    - d800-ffff

    paged:
    - display list 1 KiB
    - screen data 4 KiB
    */

    .label antic = $d400 {
        .label dmactl = $d400 // .W
        .label chactl = $d401 // .W
        .label dlist = $d402 {
            .label lo = $d402 // .W
            .label hi = $d403 // .W
        }
        .label hscrol = $d404 // .W
        .label vscrol = $d405 // .W
        .label unused_x6 = $d406 //
        .label pmbase = $d407 // .W
        .label unused_x8 = $d408 //
        .label chbase = $d409 // .W
        .label wsync = $d40a // .W
        .label vcount = $d40b // R.
        .label penh = $d40c // R.
        .label penv = $d40d // R.
        .label nmien = $d40e // .W
        .label nmist = $d40f // R.
        .label nmires = $d40f // .W
    }

    .label gtia = $d000 {
        .label m0pf = $d000 // R.
        .label hposp0 = $d000 // .W
        .label m1pf = $d001 // R.
        .label hposp1 = $d001 // .W
        .label m2pf = $d002 // R.
        .label hposp2 = $d002 // .W
        .label m3pf = $d003 // R.
        .label hposp3 = $d003 // .W
        .label p0pf = $d004 // R.
        .label hposm0 = $d004 // .W
        .label p1pf = $d005 // R.
        .label hposm1 = $d005 // .W
        .label p2pf = $d006 // R.
        .label hposm2 = $d006 // .W
        .label p3pf = $d007 // R.
        .label hposm3 = $d007 // .W
        .label m0pl = $d008 // R.
        .label sizep0 = $d008 // .W
        .label m1pl = $d009 // R.
        .label sizep1 = $d009 // .W
        .label m2pl = $d00a // R.
        .label sizep2 = $d00a // .W
        .label m3pl = $d00b // R.
        .label sizep3 = $d00b // .W
        .label p0pl = $d00c // R.
        .label sizem = $d00c // .W
        .label p1pl = $d00d // R.
        .label grafp0 = $d00d // .W
        .label p2pl = $d00e // R.
        .label grafp1 = $d00e // .W
        .label p3pl = $d00f // R.
        .label grafp2 = $d00f // .W
        .label trig0 = $d010 // R.
        .label grafp3 = $d010 // .W
        .label trig1 = $d011 // R.
        .label grafm = $d011 // .W
        .label trig2 = $d012 // R.
        .label colpm0 = $d012 // .W
        .label trig3 = $d013 // R.
        .label colpm1 = $d013 // .W
        .label pal = $d014 // R.
        .label colpm2 = $d014 // .W
        .label colpm3 = $d015 // .W
        .label colpf0 = $d016 // .W
        .label colpf1 = $d017 // .W
        .label colpf2 = $d018 // .W
        .label colpf3 = $d019 // .W
        .label colbk = $d01a // .W
        .label prior = $d01b // .W
        .label vdelay = $d01c // .W
        .label gractl = $d01d // .W
        .label hitclr = $d01e // .W
        .label consol = $d01f // RW
    }

    .label pia = $d300 {
        .label piba = $d300 // R.
        .label ora = $d300 // .W
        .label ddra = $d300 // RW
        .label pibb = $d301 // R.
        .label orb = $d301 // .W
        .label ddrb = $d301 // RW
        .label cra = $d302 //
        .label crb = $d303 //
    }

    .label pokey = $d200 {
        .label pot0 = $d200 // R.
        .label audf1 = $d200 // .W
        .label pot1 = $d201 // R.
        .label audc1 = $d201 // .W
        .label pot2 = $d202 // R.
        .label audf2 = $d202 // .W
        .label pot3 = $d203 // R.
        .label audc2 = $d203 // .W
        .label pot4 = $d204 // R.
        .label audf3 = $d204 // .W
        .label pot5 = $d205 // R.
        .label audc3 = $d205 // .W
        .label pot6 = $d206 // R.
        .label audf4 = $d206 // .W
        .label pot7 = $d207 // R.
        .label audc4 = $d207 // .W
        .label potst = $d208 // R.
        .label audctl = $d208 // .W
        .label kbcode = $d209 // R.
        .label stimer = $d209 // .W
        .label random = $d20a // R.
        .label skstres = $d20a // .W
        .label potgo = $d20b // .W
        .label unused_xc = $d20c //
        .label serin = $d20d // R.
        .label serout = $d20d // .W
        .label irqst = $d20e // R.
        .label irqen = $d20e // .W
        .label skstat = $d20f // R.
        .label skctl = $d20f // .W
    }

/* kernel::constants.rs */

use core::str::*;

pub static PROMPT: &'static str = &"\nsgash> ";

pub static SPLASH: &'static str = &"\
	\n                                                               \
	\n                                                               \
	\n                       7=..~$=..:7                             \
	\n                  +$: =$$$+$$$?$$$+ ,7?                        \
	\n                  $$$$$$$$$$$$$$$$$$Z$$                        \
	\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    \
	\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 \
	\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                \
	\n           Z$$$$$?                    .+ZZZZ$$                 \
	\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            \
	\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              \
	\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           \
	\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           \
	\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            \
	\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          \
	\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          \
	\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           \
	\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          \
	\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         \
	\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           \
	\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          \
	\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          \
	\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            \
	\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           \
	\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           \
	\n        IZZZZZ:.                        . ,ZZZZZ$              \
	\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             \
	\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 \
	\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                \
	\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                \
	\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    \
	\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    \
	\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        \
	\n                       $7..I$7..I$,                            \
	\n\
	\n _                     _     _                         _  \
	\n| |                   (_)   | |                       | | \
	\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | \
	\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | \
	\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | \
	\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\
	\n\n";
use std::collections::HashMap;

pub(crate) fn run() {
    let _input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
    let _input = _get_input();

    let message = decode(_input, true);
    println!("the message is {}", message);
}

fn decode(input: &str, get_least_common: bool) -> String {
    let lines: Vec<&str> = input.split('\n').collect();
    let len = lines.iter().next().unwrap().len();
    assert!(lines.iter().all(|line| line.len() == len));

    let mut result = String::new();
    for i in 0..len {
        let mut counts: Vec<(char, usize)> = lines
            .iter()
            .fold(HashMap::new(), |mut map, next| {
                let c = next.chars().nth(i).unwrap();
                *map.entry(c).or_default() += 1usize;
                map
            })
            .into_iter()
            .collect();
        if get_least_common {
            counts.sort_by(|&a, &b| a.1.cmp(&b.1));
        } else {
            counts.sort_by(|&a, &b| b.1.cmp(&a.1));
        }
        result.push(counts[0].0);
    }

    result
}

fn _get_input() -> &'static str {
    "ewqplnag
qchqvvsf
jdhaqbeu
jsgoijzv
iwgxjyxi
yzeeuwoi
gmgisfmd
vdtezvan
secfljup
dngzexve
xzanwmgd
ziobunnv
ennaiqiz
jgrnzpzi
huwrnmnw
qeibstlk
qegqmijn
gpwfokjg
gsmfeqmm
hlytxgti
idyjagzt
mlaztojc
xqokrslk
gkigkibl
feobhvwi
xxylgrpe
uivfbbbz
lekmcifg
ngcwvese
tgyvzlkg
pysjumnt
bmeepsda
svbznlid
hcwlvtyg
tjdzsiqu
cadtkxut
msirmtzs
cxgahqib
dtfdkzss
nrnodqjy
ptwvbmtq
ywkqyulp
ciszkcnx
ahxtwnnk
dlwvcsfc
uewwndje
ocdgocqk
auanolri
pfqyjyja
uypwzjjo
zaidpezv
tjtwiftf
fnrzsyhp
hfyqsxfu
nigauqsd
xonhbtpx
wcjciqgn
kdgvmzox
zbweztcm
irrmwyux
zmqblmfm
chcqxrqm
qjnahphi
hvkxgyeu
uqcsxxix
lhzkoydb
oyukomwi
prjjkctn
nsjvcthj
bivdsubf
galbvbof
emrnviig
bnpuofwt
shsutaeo
xkhargbp
swunowfn
dzohfvtr
kbsvqoor
dtkjgajx
bcjgfstl
jlgouhim
xmbqsvjx
brcvmnqc
eyphcrec
flnxhhzm
blrixjdy
msxlfmop
eaawcbkp
mgxiemxl
pfxtpuvh
vulefkxn
tlxfigbc
iktsstzd
qdycwpat
yjfhllyu
mmcxxloe
xpwpjnuy
sziveuyv
rmkmyqyl
hqywtzhu
pouceqty
kvfdzahj
ltiledbc
pcajwpht
kcsxqksn
bfmdmqyf
luxbaaqq
nptsvniz
aawfrzxw
keeyyptq
ryicuhie
yjvclzac
bveorbeo
ohmbvpmu
cvxejdwb
ziyffdnx
gwjxdbaq
unnrfnqh
kvicaaai
jkkiuvxj
cjviyayl
drbielvb
nulynluk
eixugugc
fxfzuonn
ludhzktb
tmqvbqfm
nzzjdxph
ukzvvges
ejplrckc
ocawtnmd
svqsxbrf
sfdfgohg
bnjrokxk
frulcpng
fjuhbzfb
wpwytpzh
jqstbhff
wkzichey
uygpxxgb
laemchta
vgjcyumm
hhloaorn
iviwosqf
kudumnei
ntfvtoay
xcimluam
wypytwno
cqboftdd
mwfcdwzw
tgwmjxfp
jysdwspw
cnsoamld
fyznzrpo
skvorpwt
plpwsuih
aysqbwem
rutkdrnn
llxxyaqe
vfhsvtxv
lgtmtjmj
ypfcjnbb
tdvnfrtv
obpdwotj
zreanciv
mfexhuff
hodukcbc
rjqrgxgn
xpmtiaec
roavlcvt
rabhqwct
ojkdtbsz
pztezpmw
qefgwtbf
ocdtbmop
dlfgvkmh
ddpzjrqc
ounagflg
vrtrakwj
ekcrcvtl
hrvghvmq
yphmhigf
nbmwllxs
gmcfdvvw
yafshyuo
hpbrminb
lwmuprvy
rajyhedj
qtrxbxal
wcqfjvfg
pvzefquu
juizosne
qbnrfgpp
muyjpylx
ljftujim
ssrjqzhi
isolpxai
lpazyyse
znrlwzhc
tvcbgplx
ecdcsjuq
axzsjwnm
edogmygw
gfbqksky
bekioiyr
nyhxmwmx
murhyrrk
rwlfdeve
trlmfwjy
zzanjgdz
bwscvdxk
tsmrttcq
fmmizwrz
cqneezoq
dhuwkslc
jwzrdomv
wxrleoed
fivvxash
ioygsjhc
qdpwprez
tagvmlbn
pqtaqcot
bdmdrheh
pfmsjlpa
hiafczzf
ovjrntwt
eoytrczw
ekcuhuur
wmqzzebk
awczvbtm
vnxrniiy
kaayoxlt
xhjtpiju
ceffyfww
vdnoycxw
pmebcukw
swbogemw
affewhdj
inbpzraz
ttjkvylh
khiljslo
ixmjrdom
wfnmgcqr
pntkncna
ezbtngtx
dxgoiwtq
gcorhdwq
mtnxxcfn
lguoqhpp
mydgtldv
dcautedv
aqxafodz
abvyoomx
qdpyeshc
eslyxatb
sxhhruer
fyudfdpl
mvbfwmhk
upmzmdmz
rqxugbwh
lubhqmre
vhpzyerz
ljyexgma
vpshuvyr
pxvuccyv
ppesevpl
mjcyazgy
mthxasgs
zkeinsxs
emehvnsz
icawtxzi
rxrpyaji
jxoxevxd
adewmqba
jcypwkfv
wspbxbnf
sjagbbna
ubfllkvq
hsecqidv
bztzbswf
udhthpya
hbpqvrrg
glnwntfm
ghpsmdjt
fgwxpvkx
sadgtywm
ipcrkfuv
tctyqmko
livzojbr
yejzdarn
aqqnctjm
emgcphcq
nkqfubfl
qojeklqt
kvsnebgk
whbowpmx
brmttrog
dyecglha
bjhyzrqq
vtkhzeyk
loopqwmv
pycecyfy
riswpqzb
fpukakic
jbyjandt
pgmqyhho
rkovglxj
gyoamarg
zffmcdgz
vajaeirw
mewxbrpv
akullmcy
hnhhlxto
vrzuwzzd
oqudtfol
hjbadzse
pttmnoan
bgvmjudu
cfrowrpy
xapmrpde
uvoxhgwo
ogzbapqj
slkplnas
nzidxmos
ymfjsfcx
celkhenj
mjsysfzp
piduvvdb
jhjlhnai
vuqwliaq
kwxnhphe
kttkiutd
kbxdqmdi
syokthzk
hgzkmhvv
zhwusjfg
qsozuerb
obyswgci
aosbzjnt
vtriuhuy
ewwggfad
ntpassqj
ggvooetp
hhmyywmv
rzhrqkvx
zapkliel
mfrgyvgw
ziwaqzun
vdpqztyc
wgxbjzxa
azvotolg
nskteyaj
mxoustqy
wfsrmtrk
xoqecgrl
dluzpwur
lokaxykx
xyqouhxb
udaqkoqf
hbvsdvkk
omqymecg
zpdwrwin
gaaprkiw
qrljdsgr
yzqzxlsu
lwxzzesm
fogpmgrb
ahahsyet
xbshcjlp
kqjnqfns
dirbsjvo
ivvuvzde
uuktpjjo
xjyqnzuz
gocimeia
qgznojog
gliwbekp
bqgakwkl
emewklsz
nrsbhxls
ksqxptkx
qayiikzs
ypulgpll
zpgbguze
oxttgrkk
usubcozu
vfdfaqdf
aijdqnws
zrafskka
qevegolp
limniayp
ufiiffly
npadruup
euamdite
plzaivpj
akqqvlro
foknpolu
yzvvtjwz
svhqjfpq
zsceoycs
fueralpo
dmwobiiv
nwmjvxxj
qvypxtyn
ycfkrxge
bdlrfvxh
ilkjiske
nebvkegk
stclxlsh
dzcomxfy
xnqqcilu
fwtpdqok
xcwpngxi
jhzgpgmd
gxfgyecr
ifzqihyl
rtdjzika
eeqqbdrn
bcxcqoif
sxdiaauc
rwfkuhka
abixxudr
aexxbgvm
ibnckfvl
wpnguagh
ukicjzms
rjcdglsa
wbbbwedq
gszpbdcd
uuliinia
oroolcgs
dbrutctl
clhhguog
jhttewcr
nudiqqvi
onpwamga
kztklrsm
moqperyh
wrlcyfwl
hsnkrqrz
jctpxrsp
dgyjdbaj
yxamrvae
cubkcqah
yvecuhqs
vvbcmhdf
mcosktuq
uonxvxhd
zileeeyl
jxebsrqb
rvkudgsu
yiflvdar
hefezoyf
vlhprvnx
gnlmhfzj
fdzgbpei
evisboku
eiultlcz
ttrpqdch
bnujwmwg
kxkijfkb
frzqsuvg
yzbrwmhf
tbytnypt
wizbqixp
sqofdzfw
gkiddyod
tqzyncjl
vfsjagyy
xkcvhice
nkkipbzd
murubxvr
aalgekbr
qzhgpqiz
rtxmuasx
vznzbbuq
bdpaucup
byzeajgv
dpedjbke
ksmynpqq
zocacvlb
zymffjwb
cegodbwk
qggqsxoo
uziyisoh
oatngkya
caumywbn
lqbnhdpj
fszkqnop
tnhssbbg
jyltqque
uwwsazxg
mwujixlj
wrslfkst
shmhlagd
rgdphggr
korsrnbu
rzjnunxy
rnjypyeo
gtvnifwz
uapadqvb
ovipnngd
dkehomjw
eaiejnmq
jeikkciu
oftckfsk
klydfonj
igglmwfo
fyubwdnh
ngzkhkpd
yuglfalc
jhjuufhh
dxemyuqq
skxsfkuf
bngixdvm
ibetxweu
vhkddick
yphvckps
vsfjvfuc
yslnkljn
owpmzvtw
hwqxmdkm
xedywgaa
gxspaddo
fgtuqtzz
lmdgicyj
wormnkqh
odjjjnjs
upwsehpy
cdnoenbr
palgbqbo
cxhtopct
atyclmda
sqqsghaw
kphxnffp
snajohsd
fgoqdmya
qukeyclq
ridnraeu
xxnrgycg
ithdkict
xkkvoupr
jdxzaowb
wsrakjua
tnlfvefb
tkopftbw
fflhizvk
qlviiyxs
tqlkpdji
wbkizspo
qfcnlwzy
icnypchf
rmcrtzhx
ibghzcrx
nwjeakcj
ozubzsep
thevuhvq
drmvjqbr
zlsxyeqi
kfbaywmd
uxpkilwv
nifwejqs
yjlhwrhl
jsotkgry
tnjboxch
loaljerf
howfiujr
zmqsffwn
uqrsbamt
othunkcr
ylhkojxs
kzldescv
irwynsjs
cytlwbvv
iqvupsei
wemgrrnj
akrqrpis
vocnluer
wjnscmyh
hekwlgim
ilmqutgu
qtnurohl
cjuclgbg
yivdapow
jrbhdxku
xholfbuw
grgfxaho
lquojibn
cbdendkb
mdurkdvz
dqdixboo
wvopazpt
xbxclroc
zjxgejjk
tmbfyyvz
cosjhwru
aqwtipsw
pmympjrh"
}

use crate::math::{ field };
use crate::{
    BASE_CYCLE_LENGTH as NUM_ROUNDS,
    SPONGE_WIDTH as STATE_WIDTH,
};

// ACCUMULATOR FUNCTIONS
// ================================================================================================

/// Executes a modified version of [Rescue](https://eprint.iacr.org/2019/426) round where inputs
/// are injected into the sate in the middle of the round. This modification differs significantly
/// form how the function was originally designed, and may potentially be insecure.
pub fn apply_round(state: &mut [u128], op_code: u128, op_value: u128, step: usize)
{
    let ark_idx = step % NUM_ROUNDS;
    
    // apply first half of Rescue round
    add_constants(state, ark_idx, 0);
    apply_sbox(state);
    apply_mds(state);

    // inject value into the state
    state[0] = field::add(state[0], op_code);
    state[1] = field::add(state[1], op_value);

    // apply second half of Rescue round
    add_constants(state, ark_idx, STATE_WIDTH);
    apply_inv_sbox(state);
    apply_mds(state);
}

pub fn add_constants(state: &mut[u128], idx: usize, offset: usize) {
    for i in 0..STATE_WIDTH {
        state[i] = field::add(state[i], ARK[offset + i][idx]);
    }
}

pub fn apply_sbox(state: &mut [u128]) {
    for i in 0..STATE_WIDTH {
        state[i] = field::exp(state[i], ALPHA);
    }
}

pub fn apply_inv_sbox(state: &mut[u128]) {
    // TODO: optimize
    for i in 0..STATE_WIDTH {
        state[i] = field::exp(state[i], INV_ALPHA);
    }
}

pub fn apply_mds(state: &mut[u128]) {
    let mut result = [field::ZERO; STATE_WIDTH];
    let mut temp = [field::ZERO; STATE_WIDTH];
    for i in 0..STATE_WIDTH {
        for j in 0..STATE_WIDTH {
            temp[j] = field::mul(MDS[i * STATE_WIDTH + j], state[j]);
        }

        for j in 0..STATE_WIDTH {
            result[i] = field::add(result[i], temp[j]);
        }
    }
    state.copy_from_slice(&result);
}

pub fn apply_inv_mds(state: &mut[u128]) {
    let mut result = [field::ZERO; STATE_WIDTH];
    let mut temp = [field::ZERO; STATE_WIDTH];
    for i in 0..STATE_WIDTH {
        for j in 0..STATE_WIDTH {
            temp[j] = field::mul(INV_MDS[i * STATE_WIDTH + j], state[j]);
        }

        for j in 0..STATE_WIDTH {
            result[i] = field::add(result[i], temp[j]);
        }
    }
    state.copy_from_slice(&result);
}

// 128-BIT RESCUE CONSTANTS
// ================================================================================================
const ALPHA: u128 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;

const MDS: [u128; STATE_WIDTH * STATE_WIDTH] = [
    315189521614069403867817270152032075784,  10737242274749505456268020883296531251, 164166492670388427786346110319108935134, 282318813916891806489021925524031494414,
    339659984245804546554434478921876908973,  97319381058524916656000376979320858814, 141017807671871944240242749183803053011, 271669633517564511702965675947905678154,
    330029911818464578106380298339390343164,  37351365266361901988170671462637236976, 260386862860725098262319886102680637202, 161805458319902660573511017706877187889,
    220775330812333365987157544144955631442, 172992909845374020745323861886602514703,   8447293670850292346208742365584924315, 276315004099450287580088164954743181255,
];

const INV_MDS: [u128; STATE_WIDTH * STATE_WIDTH] = [
    212015899302823985314659753132599968692, 222079945358547787481366483464725880498, 313947036552775452548888741999726656951,  94528877516599685906969597450601957552,
    201841258819571375352239737215387725848,  42276963631701875238524357392500799145, 332890116061360870847041499810748569092,   3939991425276748394854935956419873430,
    239100689228321601709623770733501932352, 178946314809288623489527367841505752988, 270128331008291756180543638308504150653, 315661002876081483102676309387501623498,
    298377528588644746682709801175581650901, 114666605273067789843953739274063213369, 279054651722812961169783459501878203576, 308067640269163823896854342618197051588,
];

pub const ARK: [[u128; NUM_ROUNDS]; STATE_WIDTH * 2] = [
    [ 73742662193393629993182617210984534396, 190338348930091047298074165559397264378, 135862987622353414661673448620033990934,  14395595548581550072136442264588359269, 178527953570703982986577498890483203023,  89333775516890774827962437297764936547,  60517382118002481956993039132628798754, 300207915911051460908298688414919921093, 287288998844960276649854461883880913666,   4363347423120340347647334422662129280, 169061616865327291005064664270275836534,  55063854082489962294956447144901184837,  48405253030503584410290697712994785780,  26236509279945457822369793146288866403,   8168599451814692118441936734435571667, 315851285839738308287329276161693313425],
    [170640173476284978302806154399958141555, 225556280578098393163620719229418290860,  43697512293048123577843997788308773455, 334227022756371766478760448625337379424, 188323096432976273265052369652285099186,  23833044413239455428827669432473543240, 258001239441974384951891541079242930440, 219177966622498447376602481443936826442, 294241649061853322876594266104693176711, 179443458614881887600494128053111694648, 171502007855719014010389694111716628578, 122453723578185362799857252115182955415,  97063282200318501142854934314343169049, 154737674033120948700227987365296907637, 118224404177203231307646344308524770691,  67833038363599207475373040930824843019],
    [158538539401072639862099558319550076686, 289278996656706117461857789813498821934, 158907965876520949616863328303176330572,  58496669788416466040038464653643977917, 126000924558481152083098962591383883438,  32424193637360906576442956294452323288, 337725857612570850944445340416668827103, 172066229584406173063202914726937339958, 138628264101912804813977210615833233437,  50018412546799023168899671792323407156,  16989575615175240495557720305287640349,  69216162599706897556278776240900218374,  41491163124497803255407972080635378902, 297928660776980173370496618733852490961, 233141108584353453034002234415979233911, 193135973972933518870828237886863798021],
    [  2818165229115014774032882127170013258,  11153792801390339798262783617007369172, 138405289600908304802269329797084135857, 249369722351358137898587699909312963803, 263157893448998999306850171729945394432,  43421407022492486112194101527865465264, 323117003802246814764810890058143344905, 267697496976874759865163284761384997437, 116358578177298194933445426886059838431, 339223760157195739332845857285008200423, 185875552438512768131393810027777987752, 228752352631998151610775212885524543283,  96675618862527967726114378655626650641, 253746202714926890236889780444552427226,  61911644581679112386499312030413349074, 196910153233132861881308509456401645140],
    [ 82429262549299942290847183493004485261,  53265540956785335308970867946461681393, 274633988293091071340356635555807179190, 189653807408664613044858917026657980625, 122001776241989922016768881111033630021,  62033181748425106711292370817969454146, 151495710594170316099539790651453416361, 242872884766759335785324964049644229294, 279904213525927510712810228623902377237,  78065099645540746831460653583134588104,  36224510031696203479366212612960872957, 109862755442048596627938134642975399668, 331958862978706756999748973740992156929,  81691558114273932307586556761543100315,  12234569840122312404615178877814773825, 166002344081927954304873771936867289851],
    [326468515245013538774703881972225680443, 209040028248304238735923683513240525194, 234470815157983004947611441850027217492, 311182552853825261047305944842224924215,  25509259982013669682461356932775370545,  77086595049850596660690999278719011720,   7640791703119561504971867271087353186, 170024582242541755392979256646565617273, 153964862116746563988492365899737226989,  37163237225742447359704121711857363416, 108165142884901978856319583750672324489,  69476260396969790693146402021744933499,  45955200056324872841369110391855073949, 261286087759526359216271155361018330507, 321756280164272289841871040803703440350, 334905318181122708043147970432770442813],
    [310538827479436149892724250590698914519, 221096166077280180974764042888991644280, 274604860873273636237081114376077113475, 230609671293877243511889006223284127479,  59235259390239124162891762278360245334, 129877116445533126989528570413807277693, 250107916917535224528378129994943394294, 232074846252364869196809445831737773796, 298530663250990395227144225232608384365, 265168486075436613449458788630803272512, 166545598284411242433605578379265360252, 102835474498154050313290986853294842906, 189445283838085809052254029811407633258, 302719082300742526890675313445319567341,  96037481352786813748421760769380383926, 214406010671246827947835794343033790693],
    [230635877078223923040415038811686445073, 293027053537479076557105009345927645442, 118114082982223329826045602989947510129, 185089342855265166563915858025522983409, 300544292992993952360719000252205715076, 284751400376525550861233017183497639371,  62365388436267647533064120634464266870, 243579355010018669877160932197352017974,  93028746118909893246237533845189074002, 161426242690584918941198733450953748769, 208632865147351209340449219082125897333, 185941035889835671403097747661105595079, 182846621472767704751329603405195985261, 162892536327655189685235410342890574896, 101396399019525872501663112616210307683, 191090374127295994022314014407997806335],
];
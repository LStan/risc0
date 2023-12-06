use ark_bn254::Bn254;
use ark_groth16::{prepare_verifying_key, PreparedVerifyingKey, VerifyingKey};

use crate::{convert_g1, convert_g2, from_u256};

const ALPHA_X: &str =
    "20491192805390485299153009773594534940189261866228447918068658471970481763042";
const ALPHA_Y: &str =
    "9383485363053290200918347156157836566562967994039712273449902621266178545958";
const BETA_X1: &str =
    "4252822878758300859123897981450591353533073413197771768651442665752259397132";
const BETA_X2: &str =
    "6375614351688725206403948262868962793625744043794305715222011528459656738731";
const BETA_Y1: &str =
    "21847035105528745403288232691147584728191162732299865338377159692350059136679";
const BETA_Y2: &str =
    "10505242626370262277552901082094356697409835680220590971873171140371331206856";
const GAMMA_X1: &str =
    "11559732032986387107991004021392285783925812861821192530917403151452391805634";
const GAMMA_X2: &str =
    "10857046999023057135944570762232829481370756359578518086990519993285655852781";
const GAMMA_Y1: &str =
    "4082367875863433681332203403145435568316851327593401208105741076214120093531";
const GAMMA_Y2: &str =
    "8495653923123431417604973247489272438418190587263600148770280649306958101930";
const DELTA_X1: &str =
    "18518940221910320856687047018635785128750837022059566906616608708313475199865";
const DELTA_X2: &str =
    "9492326610711013918333865133991413442330971822743127449106067493230447878125";
const DELTA_Y1: &str =
    "19483644759748826533215810634368877792922012485854314246298395665859158607201";
const DELTA_Y2: &str =
    "21375251776817431660251933179512026180139877181625068362970095925425149918084";

const IC0_X: &str = "5283414572476013565779278723585415063371186194506872223482170607932178811733";
const IC0_Y: &str = "18704069070102836155408936676819275373965966640372164023392964533091458933020";
const IC1_X: &str = "4204832149120840018317309580010992142700029278901617154852760187580780425598";
const IC1_Y: &str = "12454324579480242399557363837918019584959512625719173397955145140913291575910";
const IC2_X: &str = "14956117485756386823219519866025248834283088288522682527835557402788427995664";
const IC2_Y: &str = "6968527870554016879785099818512699922114301060378071349626144898778340839382";
const IC3_X: &str = "6512168907754184210144919576616764035747139382744482291187821746087116094329";
const IC3_Y: &str = "17156131719875889332084290091263207055049222677188492681713268727972722760739";
const IC4_X: &str = "5195346330747727606774560791771406703229046454464300598774280139349802276261";
const IC4_Y: &str = "16279160127031959334335024858510026085227931356896384961436876214395869945425";

pub fn pvk() -> Result<PreparedVerifyingKey<Bn254>, anyhow::Error> {
    let alpha_g1 = convert_g1(&vec![from_u256(ALPHA_X)?, from_u256(ALPHA_Y)?])?;
    let beta_g2 = convert_g2(&vec![
        vec![from_u256(BETA_X1)?, from_u256(BETA_X2)?],
        vec![from_u256(BETA_Y1)?, from_u256(BETA_Y2)?],
    ])?;
    let gamma_g2 = convert_g2(&vec![
        vec![from_u256(GAMMA_X1)?, from_u256(GAMMA_X2)?],
        vec![from_u256(GAMMA_Y1)?, from_u256(GAMMA_Y2)?],
    ])?;
    let delta_g2 = convert_g2(&vec![
        vec![from_u256(DELTA_X1)?, from_u256(DELTA_X2)?],
        vec![from_u256(DELTA_Y1)?, from_u256(DELTA_Y2)?],
    ])?;

    let ic0 = convert_g1(&vec![from_u256(IC0_X)?, from_u256(IC0_Y)?])?;
    let ic1 = convert_g1(&vec![from_u256(IC1_X)?, from_u256(IC1_Y)?])?;
    let ic2 = convert_g1(&vec![from_u256(IC2_X)?, from_u256(IC2_Y)?])?;
    let ic3 = convert_g1(&vec![from_u256(IC3_X)?, from_u256(IC3_Y)?])?;
    let ic4 = convert_g1(&vec![from_u256(IC4_X)?, from_u256(IC4_Y)?])?;
    let gamma_abc_g1 = vec![ic0, ic1, ic2, ic3, ic4];

    let vk = VerifyingKey::<Bn254> {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1,
    };

    Ok(prepare_verifying_key(&vk))
}

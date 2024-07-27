use crate::num::NonZeroPow2Usize;
use crate::types::BuiltinAlias::*;
use crate::types::UIntType::*;
use crate::types::*;

use simplicity::jet::Elements;

fn tuple<A: Into<AliasedType>, I: IntoIterator<Item = A>>(elements: I) -> AliasedType {
    AliasedType::tuple(elements.into_iter().map(A::into))
}

fn array<A: Into<AliasedType>>(element: A, size: usize) -> AliasedType {
    AliasedType::array(element.into(), size)
}

fn list<A: Into<AliasedType>>(element: A, bound: usize) -> AliasedType {
    AliasedType::list(element.into(), NonZeroPow2Usize::new(bound).unwrap())
}

fn bool() -> AliasedType {
    AliasedType::boolean()
}

fn either<A: Into<AliasedType>, B: Into<AliasedType>>(left: A, right: B) -> AliasedType {
    AliasedType::either(left.into(), right.into())
}

fn option<A: Into<AliasedType>>(inner: A) -> AliasedType {
    AliasedType::option(inner.into())
}

pub fn source_type(jet: Elements) -> Vec<AliasedType> {
    match jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Elements::Low1
        | Elements::Low8
        | Elements::Low16
        | Elements::Low32
        | Elements::Low64
        | Elements::High1
        | Elements::High8
        | Elements::High16
        | Elements::High32
        | Elements::High64 => vec![],
        Elements::Verify => vec![bool()],
        Elements::Complement1
        | Elements::Some1
        | Elements::LeftPadLow1_8
        | Elements::LeftPadLow1_16
        | Elements::LeftPadLow1_32
        | Elements::LeftPadLow1_64
        | Elements::LeftPadHigh1_8
        | Elements::LeftPadHigh1_16
        | Elements::LeftPadHigh1_32
        | Elements::LeftPadHigh1_64
        | Elements::LeftExtend1_8
        | Elements::LeftExtend1_16
        | Elements::LeftExtend1_32
        | Elements::LeftExtend1_64
        | Elements::RightPadLow1_8
        | Elements::RightPadLow1_16
        | Elements::RightPadLow1_32
        | Elements::RightPadLow1_64
        | Elements::RightPadHigh1_8
        | Elements::RightPadHigh1_16
        | Elements::RightPadHigh1_32
        | Elements::RightPadHigh1_64 => vec![U1.into()],
        Elements::Complement8
        | Elements::Some8
        | Elements::All8
        | Elements::Leftmost8_1
        | Elements::Leftmost8_2
        | Elements::Leftmost8_4
        | Elements::Rightmost8_1
        | Elements::Rightmost8_2
        | Elements::Rightmost8_4
        | Elements::LeftPadLow8_16
        | Elements::LeftPadLow8_32
        | Elements::LeftPadLow8_64
        | Elements::LeftPadHigh8_16
        | Elements::LeftPadHigh8_32
        | Elements::LeftPadHigh8_64
        | Elements::LeftExtend8_16
        | Elements::LeftExtend8_32
        | Elements::LeftExtend8_64
        | Elements::RightPadLow8_16
        | Elements::RightPadLow8_32
        | Elements::RightPadLow8_64
        | Elements::RightPadHigh8_16
        | Elements::RightPadHigh8_32
        | Elements::RightPadHigh8_64
        | Elements::RightExtend8_16
        | Elements::RightExtend8_32
        | Elements::RightExtend8_64 => vec![U8.into()],
        Elements::Complement16
        | Elements::Some16
        | Elements::All16
        | Elements::Leftmost16_1
        | Elements::Leftmost16_2
        | Elements::Leftmost16_4
        | Elements::Leftmost16_8
        | Elements::Rightmost16_1
        | Elements::Rightmost16_2
        | Elements::Rightmost16_4
        | Elements::Rightmost16_8
        | Elements::LeftPadLow16_32
        | Elements::LeftPadLow16_64
        | Elements::LeftPadHigh16_32
        | Elements::LeftPadHigh16_64
        | Elements::LeftExtend16_32
        | Elements::LeftExtend16_64
        | Elements::RightPadLow16_32
        | Elements::RightPadLow16_64
        | Elements::RightPadHigh16_32
        | Elements::RightPadHigh16_64
        | Elements::RightExtend16_32
        | Elements::RightExtend16_64 => vec![U16.into()],
        Elements::Complement32
        | Elements::Some32
        | Elements::All32
        | Elements::Leftmost32_1
        | Elements::Leftmost32_2
        | Elements::Leftmost32_4
        | Elements::Leftmost32_8
        | Elements::Leftmost32_16
        | Elements::Rightmost32_1
        | Elements::Rightmost32_2
        | Elements::Rightmost32_4
        | Elements::Rightmost32_8
        | Elements::Rightmost32_16
        | Elements::LeftPadLow32_64
        | Elements::LeftPadHigh32_64
        | Elements::LeftExtend32_64
        | Elements::RightPadLow32_64
        | Elements::RightPadHigh32_64
        | Elements::RightExtend32_64 => vec![U32.into()],
        Elements::Complement64
        | Elements::Some64
        | Elements::All64
        | Elements::Leftmost64_1
        | Elements::Leftmost64_2
        | Elements::Leftmost64_4
        | Elements::Leftmost64_8
        | Elements::Leftmost64_16
        | Elements::Leftmost64_32
        | Elements::Rightmost64_1
        | Elements::Rightmost64_2
        | Elements::Rightmost64_4
        | Elements::Rightmost64_8
        | Elements::Rightmost64_16
        | Elements::Rightmost64_32 => vec![U64.into()],
        Elements::And1 | Elements::Or1 | Elements::Xor1 | Elements::Eq1 => {
            vec![U1.into(), U1.into()]
        }
        Elements::And8 | Elements::Or8 | Elements::Xor8 | Elements::Eq8 => {
            vec![U8.into(), U8.into()]
        }
        Elements::And16 | Elements::Or16 | Elements::Xor16 | Elements::Eq16 => {
            vec![U16.into(), U16.into()]
        }
        Elements::And32 | Elements::Or32 | Elements::Xor32 | Elements::Eq32 => {
            vec![U32.into(), U32.into()]
        }
        Elements::And64 | Elements::Or64 | Elements::Xor64 | Elements::Eq64 => {
            vec![U64.into(), U64.into()]
        }
        Elements::Eq256 => vec![U256.into(), U256.into()],
        Elements::Maj1 | Elements::XorXor1 | Elements::Ch1 => vec![U1.into(), U1.into(), U1.into()],
        Elements::Maj8 | Elements::XorXor8 | Elements::Ch8 => vec![U8.into(), U8.into(), U8.into()],
        Elements::Maj16 | Elements::XorXor16 | Elements::Ch16 => {
            vec![U16.into(), tuple([U16, U16])]
        }
        Elements::Maj32 | Elements::XorXor32 | Elements::Ch32 => {
            vec![U32.into(), tuple([U32, U32])]
        }
        Elements::Maj64 | Elements::XorXor64 | Elements::Ch64 => {
            vec![U64.into(), tuple([U64, U64])]
        }
        Elements::FullLeftShift8_1 => vec![U8.into(), U1.into()],
        Elements::FullLeftShift8_2 => vec![U8.into(), U2.into()],
        Elements::FullLeftShift8_4 => vec![U8.into(), U4.into()],
        Elements::FullLeftShift16_1 => vec![U16.into(), U1.into()],
        Elements::FullLeftShift16_2 => vec![U16.into(), U2.into()],
        Elements::FullLeftShift16_4 => vec![U16.into(), U4.into()],
        Elements::FullLeftShift16_8 => vec![U16.into(), U8.into()],
        Elements::FullLeftShift32_1 => vec![U32.into(), U1.into()],
        Elements::FullLeftShift32_2 => vec![U32.into(), U2.into()],
        Elements::FullLeftShift32_4 => vec![U32.into(), U4.into()],
        Elements::FullLeftShift32_8 => vec![U32.into(), U8.into()],
        Elements::FullLeftShift32_16 => vec![U32.into(), U16.into()],
        Elements::FullLeftShift64_1 => vec![U64.into(), U1.into()],
        Elements::FullLeftShift64_2 => vec![U64.into(), U2.into()],
        Elements::FullLeftShift64_4 => vec![U64.into(), U4.into()],
        Elements::FullLeftShift64_8 => vec![U64.into(), U8.into()],
        Elements::FullLeftShift64_16 => vec![U64.into(), U16.into()],
        Elements::FullLeftShift64_32 => vec![U64.into(), U32.into()],
        Elements::FullRightShift8_1 => vec![U1.into(), U8.into()],
        Elements::FullRightShift8_2 => vec![U2.into(), U8.into()],
        Elements::FullRightShift8_4 => vec![U4.into(), U8.into()],
        Elements::FullRightShift16_1 => vec![U1.into(), U16.into()],
        Elements::FullRightShift16_2 => vec![U2.into(), U16.into()],
        Elements::FullRightShift16_4 => vec![U4.into(), U16.into()],
        Elements::FullRightShift16_8 => vec![U8.into(), U16.into()],
        Elements::FullRightShift32_1 => vec![U1.into(), U32.into()],
        Elements::FullRightShift32_2 => vec![U2.into(), U32.into()],
        Elements::FullRightShift32_4 => vec![U4.into(), U32.into()],
        Elements::FullRightShift32_8 => vec![U8.into(), U32.into()],
        Elements::FullRightShift32_16 => vec![U16.into(), U32.into()],
        Elements::FullRightShift64_1 => vec![U1.into(), U64.into()],
        Elements::FullRightShift64_2 => vec![U2.into(), U64.into()],
        Elements::FullRightShift64_4 => vec![U4.into(), U64.into()],
        Elements::FullRightShift64_8 => vec![U8.into(), U64.into()],
        Elements::FullRightShift64_16 => vec![U16.into(), U64.into()],
        Elements::FullRightShift64_32 => vec![U32.into(), U64.into()],
        Elements::LeftShiftWith8 | Elements::RightShiftWith8 => {
            vec![U1.into(), U4.into(), U8.into()]
        }
        Elements::LeftShiftWith16 | Elements::RightShiftWith16 => {
            vec![U1.into(), U4.into(), U16.into()]
        }
        Elements::LeftShiftWith32 | Elements::RightShiftWith32 => {
            vec![U1.into(), U8.into(), U32.into()]
        }
        Elements::LeftShiftWith64 | Elements::RightShiftWith64 => {
            vec![U1.into(), U8.into(), U64.into()]
        }
        Elements::LeftShift8
        | Elements::RightShift8
        | Elements::LeftRotate8
        | Elements::RightRotate8 => vec![U4.into(), U8.into()],
        Elements::LeftShift16
        | Elements::RightShift16
        | Elements::LeftRotate16
        | Elements::RightRotate16 => vec![U4.into(), U16.into()],
        Elements::LeftShift32
        | Elements::RightShift32
        | Elements::LeftRotate32
        | Elements::RightRotate32 => vec![U8.into(), U32.into()],
        Elements::LeftShift64
        | Elements::RightShift64
        | Elements::LeftRotate64
        | Elements::RightRotate64 => vec![U8.into(), U64.into()],
        /*
         * Arithmetic
         */
        Elements::One8 | Elements::One16 | Elements::One32 | Elements::One64 => vec![],
        Elements::Increment8
        | Elements::Negate8
        | Elements::Decrement8
        | Elements::IsZero8
        | Elements::IsOne8 => vec![U8.into()],
        Elements::Increment16
        | Elements::Negate16
        | Elements::Decrement16
        | Elements::IsZero16
        | Elements::IsOne16 => vec![U16.into()],
        Elements::Increment32
        | Elements::Negate32
        | Elements::Decrement32
        | Elements::IsZero32
        | Elements::IsOne32 => vec![U32.into()],
        Elements::Increment64
        | Elements::Negate64
        | Elements::Decrement64
        | Elements::IsZero64
        | Elements::IsOne64 => vec![U64.into()],
        Elements::Add8
        | Elements::Subtract8
        | Elements::Multiply8
        | Elements::Le8
        | Elements::Lt8
        | Elements::Min8
        | Elements::Max8
        | Elements::DivMod8
        | Elements::Divide8
        | Elements::Modulo8
        | Elements::Divides8 => vec![U8.into(), U8.into()],
        Elements::Add16
        | Elements::Subtract16
        | Elements::Multiply16
        | Elements::Le16
        | Elements::Lt16
        | Elements::Min16
        | Elements::Max16
        | Elements::DivMod16
        | Elements::Divide16
        | Elements::Modulo16
        | Elements::Divides16 => vec![U16.into(), U16.into()],
        Elements::Add32
        | Elements::Subtract32
        | Elements::Multiply32
        | Elements::Le32
        | Elements::Lt32
        | Elements::Min32
        | Elements::Max32
        | Elements::DivMod32
        | Elements::Divide32
        | Elements::Modulo32
        | Elements::Divides32 => vec![U32.into(), U32.into()],
        Elements::Add64
        | Elements::Subtract64
        | Elements::Multiply64
        | Elements::Le64
        | Elements::Lt64
        | Elements::Min64
        | Elements::Max64
        | Elements::DivMod64
        | Elements::Divide64
        | Elements::Modulo64
        | Elements::Divides64 => vec![U64.into(), U64.into()],
        Elements::FullAdd8 | Elements::FullSubtract8 => vec![bool(), U8.into(), U8.into()],
        Elements::FullAdd16 | Elements::FullSubtract16 => vec![bool(), U16.into(), U16.into()],
        Elements::FullAdd32 | Elements::FullSubtract32 => vec![bool(), U32.into(), U32.into()],
        Elements::FullAdd64 | Elements::FullSubtract64 => vec![bool(), U64.into(), U64.into()],
        Elements::FullIncrement8 | Elements::FullDecrement8 => vec![bool(), U8.into()],
        Elements::FullIncrement16 | Elements::FullDecrement16 => vec![bool(), U16.into()],
        Elements::FullIncrement32 | Elements::FullDecrement32 => vec![bool(), U32.into()],
        Elements::FullIncrement64 | Elements::FullDecrement64 => vec![bool(), U64.into()],
        Elements::FullMultiply8 => vec![tuple([U8, U8]), tuple([U8, U8])],
        Elements::FullMultiply16 => vec![tuple([U16, U16]), tuple([U16, U16])],
        Elements::FullMultiply32 => vec![tuple([U32, U32]), tuple([U32, U32])],
        Elements::FullMultiply64 => vec![tuple([U64, U64]), tuple([U64, U64])],
        Elements::Median8 => vec![U8.into(), U8.into(), U8.into()],
        Elements::Median16 => vec![U16.into(), U16.into(), U16.into()],
        Elements::Median32 => vec![U32.into(), U32.into(), U32.into()],
        Elements::Median64 => vec![U64.into(), U64.into(), U64.into()],
        /*
         * Hash functions
         */
        Elements::Sha256Iv | Elements::Sha256Ctx8Init => vec![],
        Elements::Sha256Block => vec![U256.into(), U256.into(), U256.into()],
        Elements::Sha256Ctx8Add1 => vec![Ctx8.into(), U8.into()],
        Elements::Sha256Ctx8Add2 => vec![Ctx8.into(), U16.into()],
        Elements::Sha256Ctx8Add4 => vec![Ctx8.into(), U32.into()],
        Elements::Sha256Ctx8Add8 => vec![Ctx8.into(), U64.into()],
        Elements::Sha256Ctx8Add16 => vec![Ctx8.into(), U128.into()],
        Elements::Sha256Ctx8Add32 => vec![Ctx8.into(), U256.into()],
        Elements::Sha256Ctx8Add64 => vec![Ctx8.into(), array(U8, 64)],
        Elements::Sha256Ctx8Add128 => vec![Ctx8.into(), array(U8, 128)],
        Elements::Sha256Ctx8Add256 => vec![Ctx8.into(), array(U8, 256)],
        Elements::Sha256Ctx8Add512 => vec![Ctx8.into(), array(U8, 512)],
        Elements::Sha256Ctx8AddBuffer511 => vec![Ctx8.into(), list(U8, 512)],
        Elements::Sha256Ctx8Finalize => vec![Ctx8.into()],
        /*
         * Elliptic curve functions
         */
        // XXX: Nonstandard tuple
        Elements::PointVerify1 => {
            vec![tuple([tuple([Scalar, Point]), Scalar.into()]), Point.into()]
        }
        Elements::Decompress => vec![Point.into()],
        // XXX: Nonstandard tuple
        Elements::LinearVerify1 => vec![tuple([tuple([Scalar, Ge]), Scalar.into()]), Ge.into()],
        // XXX: Nonstandard tuple
        Elements::LinearCombination1 => vec![tuple([Scalar, Gej]), Scalar.into()],
        Elements::Scale => vec![Scalar.into(), Gej.into()],
        Elements::Generate => vec![Scalar.into()],
        Elements::GejInfinity => vec![],
        Elements::GejNormalize
        | Elements::GejNegate
        | Elements::GejDouble
        | Elements::GejIsInfinity
        | Elements::GejYIsOdd
        | Elements::GejIsOnCurve => vec![Gej.into()],
        Elements::GeNegate | Elements::GeIsOnCurve => vec![Ge.into()],
        Elements::GejAdd | Elements::GejEquiv => vec![Gej.into(), Gej.into()],
        Elements::GejGeAddEx | Elements::GejGeAdd | Elements::GejGeEquiv => {
            vec![Gej.into(), Ge.into()]
        }
        Elements::GejRescale => vec![Gej.into(), Fe.into()],
        Elements::GejXEquiv => vec![Fe.into(), Gej.into()],
        Elements::ScalarAdd | Elements::ScalarMultiply => vec![Scalar.into(), Scalar.into()],
        Elements::ScalarNormalize
        | Elements::ScalarNegate
        | Elements::ScalarSquare
        | Elements::ScalarInvert
        | Elements::ScalarMultiplyLambda
        | Elements::ScalarIsZero => vec![Scalar.into()],
        Elements::FeNormalize
        | Elements::FeNegate
        | Elements::FeSquare
        | Elements::FeMultiplyBeta
        | Elements::FeInvert
        | Elements::FeSquareRoot
        | Elements::FeIsZero
        | Elements::FeIsOdd => vec![Fe.into()],
        Elements::FeAdd | Elements::FeMultiply => vec![Fe.into(), Fe.into()],
        /*
         * Digital signatures
         */
        // XXX: Nonstandard tuple
        Elements::CheckSigVerify => vec![tuple([Pubkey, Message64]), Signature.into()],
        // XXX: Nonstandard tuple
        Elements::Bip0340Verify => vec![tuple([Pubkey, Message]), Signature.into()],
        /*
         * Bitcoin (without primitives)
         */
        Elements::ParseLock | Elements::ParseSequence => vec![U32.into()],
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        Elements::SigAllHash
        | Elements::TxHash
        | Elements::TapEnvHash
        | Elements::InputsHash
        | Elements::OutputsHash
        | Elements::IssuancesHash
        | Elements::InputUtxosHash
        | Elements::OutputAmountsHash
        | Elements::OutputScriptsHash
        | Elements::OutputNoncesHash
        | Elements::OutputRangeProofsHash
        | Elements::OutputSurjectionProofsHash
        | Elements::InputOutpointsHash
        | Elements::InputAnnexesHash
        | Elements::InputSequencesHash
        | Elements::InputScriptSigsHash
        | Elements::IssuanceAssetAmountsHash
        | Elements::IssuanceTokenAmountsHash
        | Elements::IssuanceRangeProofsHash
        | Elements::IssuanceBlindingEntropyHash
        | Elements::InputAmountsHash
        | Elements::InputScriptsHash
        | Elements::TapleafHash
        | Elements::TappathHash => vec![],
        Elements::OutpointHash => vec![Ctx8.into(), option(U256), Outpoint.into()],
        Elements::AssetAmountHash => {
            vec![Ctx8.into(), Asset1.into(), Amount1.into()]
        }
        Elements::NonceHash => vec![Ctx8.into(), option(Nonce)],
        Elements::AnnexHash => vec![Ctx8.into(), option(U256)],
        Elements::BuildTapleafSimplicity => vec![U256.into()],
        Elements::BuildTapbranch => vec![U256.into(), U256.into()],
        /*
         * Time locks
         */
        Elements::CheckLockTime => vec![Time.into()],
        Elements::CheckLockDistance => vec![Distance.into()],
        Elements::CheckLockDuration => vec![Duration.into()],
        Elements::CheckLockHeight => vec![Height.into()],
        Elements::TxLockTime
        | Elements::TxLockDistance
        | Elements::TxLockDuration
        | Elements::TxLockHeight
        | Elements::TxIsFinal => vec![],
        /*
         * Issuance
         */
        Elements::Issuance
        | Elements::IssuanceAsset
        | Elements::IssuanceToken
        | Elements::IssuanceEntropy => vec![U32.into()],
        Elements::CalculateIssuanceEntropy => vec![Outpoint.into(), U256.into()],
        Elements::CalculateAsset
        | Elements::CalculateExplicitToken
        | Elements::CalculateConfidentialToken => vec![U256.into()],
        /*
         * Transaction
         */
        Elements::ScriptCMR
        | Elements::InternalKey
        | Elements::CurrentIndex
        | Elements::NumInputs
        | Elements::NumOutputs
        | Elements::LockTime
        | Elements::CurrentPegin
        | Elements::CurrentPrevOutpoint
        | Elements::CurrentAsset
        | Elements::CurrentAmount
        | Elements::CurrentScriptHash
        | Elements::CurrentSequence
        | Elements::CurrentAnnexHash
        | Elements::CurrentScriptSigHash
        | Elements::CurrentReissuanceBlinding
        | Elements::CurrentNewIssuanceContract
        | Elements::CurrentReissuanceEntropy
        | Elements::CurrentIssuanceTokenAmount
        | Elements::CurrentIssuanceAssetAmount
        | Elements::CurrentIssuanceAssetProof
        | Elements::CurrentIssuanceTokenProof
        | Elements::TapleafVersion
        | Elements::Version
        | Elements::GenesisBlockHash => vec![],
        Elements::OutputAsset
        | Elements::OutputAmount
        | Elements::OutputNonce
        | Elements::OutputScriptHash
        | Elements::OutputIsFee
        | Elements::OutputSurjectionProof
        | Elements::OutputRangeProof
        | Elements::InputPegin
        | Elements::InputPrevOutpoint
        | Elements::InputAsset
        | Elements::InputAmount
        | Elements::InputScriptHash
        | Elements::InputSequence
        | Elements::InputAnnexHash
        | Elements::InputScriptSigHash
        | Elements::ReissuanceBlinding
        | Elements::NewIssuanceContract
        | Elements::ReissuanceEntropy
        | Elements::IssuanceAssetAmount
        | Elements::IssuanceTokenAmount
        | Elements::IssuanceAssetProof
        | Elements::IssuanceTokenProof => vec![U32.into()],
        Elements::OutputNullDatum => vec![U32.into(), U32.into()],
        Elements::TotalFee => vec![ExplicitAsset.into()],
        Elements::Tappath => vec![U8.into()],
    }
}

pub fn target_type(jet: Elements) -> AliasedType {
    match jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Elements::Verify => AliasedType::unit(),
        Elements::Some1
        | Elements::Some8
        | Elements::Some16
        | Elements::Some32
        | Elements::Some64
        | Elements::All8
        | Elements::All16
        | Elements::All32
        | Elements::All64
        | Elements::Eq1
        | Elements::Eq8
        | Elements::Eq16
        | Elements::Eq32
        | Elements::Eq64
        | Elements::Eq256 => bool(),
        Elements::Low1
        | Elements::High1
        | Elements::Complement1
        | Elements::And1
        | Elements::Or1
        | Elements::Xor1
        | Elements::Maj1
        | Elements::XorXor1
        | Elements::Ch1
        | Elements::Leftmost8_1
        | Elements::Rightmost8_1
        | Elements::Leftmost16_1
        | Elements::Rightmost16_1
        | Elements::Leftmost32_1
        | Elements::Rightmost32_1
        | Elements::Leftmost64_1
        | Elements::Rightmost64_1 => U1.into(),
        Elements::Leftmost8_2
        | Elements::Rightmost8_2
        | Elements::Leftmost16_2
        | Elements::Rightmost16_2
        | Elements::Leftmost32_2
        | Elements::Rightmost32_2
        | Elements::Leftmost64_2
        | Elements::Rightmost64_2 => U2.into(),
        Elements::Leftmost8_4
        | Elements::Rightmost8_4
        | Elements::Leftmost16_4
        | Elements::Rightmost16_4
        | Elements::Leftmost32_4
        | Elements::Rightmost32_4
        | Elements::Leftmost64_4
        | Elements::Rightmost64_4 => U4.into(),
        Elements::Low8
        | Elements::High8
        | Elements::Complement8
        | Elements::And8
        | Elements::Or8
        | Elements::Xor8
        | Elements::Maj8
        | Elements::XorXor8
        | Elements::Ch8
        | Elements::Leftmost16_8
        | Elements::Rightmost16_8
        | Elements::Leftmost32_8
        | Elements::Rightmost32_8
        | Elements::Leftmost64_8
        | Elements::Rightmost64_8
        | Elements::LeftPadLow1_8
        | Elements::LeftPadHigh1_8
        | Elements::LeftExtend1_8
        | Elements::RightPadLow1_8
        | Elements::RightPadHigh1_8
        | Elements::LeftShiftWith8
        | Elements::RightShiftWith8
        | Elements::LeftShift8
        | Elements::RightShift8
        | Elements::LeftRotate8
        | Elements::RightRotate8 => U8.into(),
        Elements::Low16
        | Elements::High16
        | Elements::Complement16
        | Elements::And16
        | Elements::Or16
        | Elements::Xor16
        | Elements::Maj16
        | Elements::XorXor16
        | Elements::Ch16
        | Elements::Leftmost32_16
        | Elements::Rightmost32_16
        | Elements::Leftmost64_16
        | Elements::Rightmost64_16
        | Elements::LeftPadLow1_16
        | Elements::LeftPadHigh1_16
        | Elements::LeftExtend1_16
        | Elements::RightPadLow1_16
        | Elements::RightPadHigh1_16
        | Elements::LeftPadLow8_16
        | Elements::LeftPadHigh8_16
        | Elements::LeftExtend8_16
        | Elements::RightPadLow8_16
        | Elements::RightPadHigh8_16
        | Elements::RightExtend8_16
        | Elements::LeftShiftWith16
        | Elements::RightShiftWith16
        | Elements::LeftShift16
        | Elements::RightShift16
        | Elements::LeftRotate16
        | Elements::RightRotate16 => U16.into(),
        Elements::Low32
        | Elements::High32
        | Elements::Complement32
        | Elements::And32
        | Elements::Or32
        | Elements::Xor32
        | Elements::Maj32
        | Elements::XorXor32
        | Elements::Ch32
        | Elements::Leftmost64_32
        | Elements::Rightmost64_32
        | Elements::LeftPadLow1_32
        | Elements::LeftPadHigh1_32
        | Elements::LeftExtend1_32
        | Elements::RightPadLow1_32
        | Elements::RightPadHigh1_32
        | Elements::LeftPadLow8_32
        | Elements::LeftPadHigh8_32
        | Elements::LeftExtend8_32
        | Elements::RightPadLow8_32
        | Elements::RightPadHigh8_32
        | Elements::RightExtend8_32
        | Elements::LeftPadLow16_32
        | Elements::LeftPadHigh16_32
        | Elements::LeftExtend16_32
        | Elements::RightPadLow16_32
        | Elements::RightPadHigh16_32
        | Elements::RightExtend16_32
        | Elements::LeftShiftWith32
        | Elements::RightShiftWith32
        | Elements::LeftShift32
        | Elements::RightShift32
        | Elements::LeftRotate32
        | Elements::RightRotate32 => U32.into(),
        Elements::Low64
        | Elements::High64
        | Elements::Complement64
        | Elements::And64
        | Elements::Or64
        | Elements::Xor64
        | Elements::Maj64
        | Elements::XorXor64
        | Elements::Ch64
        | Elements::LeftPadLow1_64
        | Elements::LeftPadHigh1_64
        | Elements::LeftExtend1_64
        | Elements::RightPadLow1_64
        | Elements::RightPadHigh1_64
        | Elements::LeftPadLow8_64
        | Elements::LeftPadHigh8_64
        | Elements::LeftExtend8_64
        | Elements::RightPadLow8_64
        | Elements::RightPadHigh8_64
        | Elements::RightExtend8_64
        | Elements::LeftPadLow16_64
        | Elements::LeftPadHigh16_64
        | Elements::LeftExtend16_64
        | Elements::RightPadLow16_64
        | Elements::RightPadHigh16_64
        | Elements::RightExtend16_64
        | Elements::LeftPadLow32_64
        | Elements::LeftPadHigh32_64
        | Elements::LeftExtend32_64
        | Elements::RightPadLow32_64
        | Elements::RightPadHigh32_64
        | Elements::RightExtend32_64
        | Elements::LeftShiftWith64
        | Elements::RightShiftWith64
        | Elements::LeftShift64
        | Elements::RightShift64
        | Elements::LeftRotate64
        | Elements::RightRotate64 => U64.into(),
        Elements::FullLeftShift8_1 => tuple([U1, U8]),
        Elements::FullLeftShift8_2 => tuple([U2, U8]),
        Elements::FullLeftShift8_4 => tuple([U4, U8]),
        Elements::FullLeftShift16_1 => tuple([U1, U16]),
        Elements::FullLeftShift16_2 => tuple([U2, U16]),
        Elements::FullLeftShift16_4 => tuple([U4, U16]),
        Elements::FullLeftShift16_8 => tuple([U8, U16]),
        Elements::FullLeftShift32_1 => tuple([U1, U32]),
        Elements::FullLeftShift32_2 => tuple([U2, U32]),
        Elements::FullLeftShift32_4 => tuple([U4, U32]),
        Elements::FullLeftShift32_8 => tuple([U8, U32]),
        Elements::FullLeftShift32_16 => tuple([U16, U32]),
        Elements::FullLeftShift64_1 => tuple([U1, U64]),
        Elements::FullLeftShift64_2 => tuple([U2, U64]),
        Elements::FullLeftShift64_4 => tuple([U4, U64]),
        Elements::FullLeftShift64_8 => tuple([U8, U64]),
        Elements::FullLeftShift64_16 => tuple([U16, U64]),
        Elements::FullLeftShift64_32 => tuple([U32, U64]),
        Elements::FullRightShift8_1 => tuple([U8, U1]),
        Elements::FullRightShift8_2 => tuple([U8, U2]),
        Elements::FullRightShift8_4 => tuple([U8, U4]),
        Elements::FullRightShift16_1 => tuple([U16, U1]),
        Elements::FullRightShift16_2 => tuple([U16, U2]),
        Elements::FullRightShift16_4 => tuple([U16, U4]),
        Elements::FullRightShift16_8 => tuple([U16, U8]),
        Elements::FullRightShift32_1 => tuple([U32, U1]),
        Elements::FullRightShift32_2 => tuple([U32, U2]),
        Elements::FullRightShift32_4 => tuple([U32, U4]),
        Elements::FullRightShift32_8 => tuple([U32, U8]),
        Elements::FullRightShift32_16 => tuple([U32, U16]),
        Elements::FullRightShift64_1 => tuple([U64, U1]),
        Elements::FullRightShift64_2 => tuple([U64, U2]),
        Elements::FullRightShift64_4 => tuple([U64, U4]),
        Elements::FullRightShift64_8 => tuple([U64, U8]),
        Elements::FullRightShift64_16 => tuple([U64, U16]),
        Elements::FullRightShift64_32 => tuple([U64, U32]),
        /*
         * Arithmetic
         */
        Elements::Le8
        | Elements::Lt8
        | Elements::Le16
        | Elements::Lt16
        | Elements::Le32
        | Elements::Lt32
        | Elements::Le64
        | Elements::Lt64
        | Elements::IsZero8
        | Elements::IsOne8
        | Elements::IsZero16
        | Elements::IsOne16
        | Elements::IsZero32
        | Elements::IsOne32
        | Elements::IsZero64
        | Elements::IsOne64
        | Elements::Divides8
        | Elements::Divides16
        | Elements::Divides32
        | Elements::Divides64 => bool(),
        Elements::One8
        | Elements::Min8
        | Elements::Max8
        | Elements::Divide8
        | Elements::Modulo8
        | Elements::Median8 => U8.into(),
        Elements::One16
        | Elements::Min16
        | Elements::Max16
        | Elements::Divide16
        | Elements::Modulo16
        | Elements::Multiply8
        | Elements::FullMultiply8
        | Elements::Median16 => U16.into(),
        Elements::One32
        | Elements::Min32
        | Elements::Max32
        | Elements::Divide32
        | Elements::Modulo32
        | Elements::Multiply16
        | Elements::FullMultiply16
        | Elements::Median32 => U32.into(),
        Elements::One64
        | Elements::Min64
        | Elements::Max64
        | Elements::Divide64
        | Elements::Modulo64
        | Elements::Multiply32
        | Elements::FullMultiply32
        | Elements::Median64 => U64.into(),
        Elements::Multiply64 | Elements::FullMultiply64 => U128.into(),
        Elements::Increment8
        | Elements::Negate8
        | Elements::Decrement8
        | Elements::Add8
        | Elements::Subtract8
        | Elements::FullAdd8
        | Elements::FullSubtract8
        | Elements::FullIncrement8
        | Elements::FullDecrement8 => tuple([bool(), U8.into()]),
        Elements::Increment16
        | Elements::Negate16
        | Elements::Decrement16
        | Elements::Add16
        | Elements::Subtract16
        | Elements::FullAdd16
        | Elements::FullSubtract16
        | Elements::FullIncrement16
        | Elements::FullDecrement16 => tuple([bool(), U16.into()]),
        Elements::Increment32
        | Elements::Negate32
        | Elements::Decrement32
        | Elements::Add32
        | Elements::Subtract32
        | Elements::FullAdd32
        | Elements::FullSubtract32
        | Elements::FullIncrement32
        | Elements::FullDecrement32 => tuple([bool(), U32.into()]),
        Elements::Increment64
        | Elements::Negate64
        | Elements::Decrement64
        | Elements::Add64
        | Elements::Subtract64
        | Elements::FullAdd64
        | Elements::FullSubtract64
        | Elements::FullIncrement64
        | Elements::FullDecrement64 => tuple([bool(), U64.into()]),
        Elements::DivMod8 => tuple([U8, U8]),
        Elements::DivMod16 => tuple([U16, U16]),
        Elements::DivMod32 => tuple([U32, U32]),
        Elements::DivMod64 => tuple([U64, U64]),
        /*
         * Hash functions
         */
        Elements::Sha256Iv | Elements::Sha256Block | Elements::Sha256Ctx8Finalize => U256.into(),
        Elements::Sha256Ctx8Init
        | Elements::Sha256Ctx8Add1
        | Elements::Sha256Ctx8Add2
        | Elements::Sha256Ctx8Add4
        | Elements::Sha256Ctx8Add8
        | Elements::Sha256Ctx8Add16
        | Elements::Sha256Ctx8Add32
        | Elements::Sha256Ctx8Add64
        | Elements::Sha256Ctx8Add128
        | Elements::Sha256Ctx8Add256
        | Elements::Sha256Ctx8Add512
        | Elements::Sha256Ctx8AddBuffer511 => Ctx8.into(),
        /*
         * Elliptic curve functions
         */
        Elements::PointVerify1 | Elements::LinearVerify1 => AliasedType::unit(),
        Elements::GejIsInfinity
        | Elements::GejEquiv
        | Elements::GejGeEquiv
        | Elements::GejXEquiv
        | Elements::GejYIsOdd
        | Elements::GejIsOnCurve
        | Elements::GeIsOnCurve
        | Elements::ScalarIsZero
        | Elements::FeIsZero
        | Elements::FeIsOdd => bool(),
        Elements::GeNegate => Ge.into(),
        Elements::Decompress | Elements::GejNormalize => option(Ge),
        Elements::LinearCombination1
        | Elements::Scale
        | Elements::Generate
        | Elements::GejInfinity
        | Elements::GejNegate
        | Elements::GejDouble
        | Elements::GejAdd
        | Elements::GejGeAdd
        | Elements::GejRescale => Gej.into(),
        Elements::GejGeAddEx => tuple([Fe, Gej]),
        Elements::ScalarNormalize
        | Elements::ScalarNegate
        | Elements::ScalarAdd
        | Elements::ScalarSquare
        | Elements::ScalarMultiply
        | Elements::ScalarMultiplyLambda
        | Elements::ScalarInvert => Scalar.into(),
        Elements::FeNormalize
        | Elements::FeNegate
        | Elements::FeAdd
        | Elements::FeSquare
        | Elements::FeMultiply
        | Elements::FeMultiplyBeta
        | Elements::FeInvert => Fe.into(),
        Elements::FeSquareRoot => option(Fe),
        /*
         * Digital signatures
         */
        Elements::CheckSigVerify | Elements::Bip0340Verify => AliasedType::unit(),
        /*
         * Bitcoin (without primitives)
         */
        Elements::ParseLock => either(Height, Time),
        Elements::ParseSequence => option(either(Distance, Duration)),
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        Elements::SigAllHash
        | Elements::TxHash
        | Elements::TapEnvHash
        | Elements::InputsHash
        | Elements::OutputsHash
        | Elements::IssuancesHash
        | Elements::InputUtxosHash
        | Elements::OutputAmountsHash
        | Elements::OutputScriptsHash
        | Elements::OutputNoncesHash
        | Elements::OutputRangeProofsHash
        | Elements::OutputSurjectionProofsHash
        | Elements::InputOutpointsHash
        | Elements::InputAnnexesHash
        | Elements::InputSequencesHash
        | Elements::InputScriptSigsHash
        | Elements::IssuanceAssetAmountsHash
        | Elements::IssuanceTokenAmountsHash
        | Elements::IssuanceRangeProofsHash
        | Elements::IssuanceBlindingEntropyHash
        | Elements::InputAmountsHash
        | Elements::InputScriptsHash
        | Elements::TapleafHash
        | Elements::TappathHash
        | Elements::BuildTapleafSimplicity
        | Elements::BuildTapbranch => U256.into(),
        Elements::OutpointHash
        | Elements::AssetAmountHash
        | Elements::NonceHash
        | Elements::AnnexHash => Ctx8.into(),
        /*
         * Time locks
         */
        Elements::CheckLockTime
        | Elements::CheckLockDistance
        | Elements::CheckLockDuration
        | Elements::CheckLockHeight => AliasedType::unit(),
        Elements::TxIsFinal => bool(),
        Elements::TxLockTime => Time.into(),
        Elements::TxLockDistance => Distance.into(),
        Elements::TxLockDuration => Duration.into(),
        Elements::TxLockHeight => Height.into(),
        /*
         * Issuance
         */
        Elements::Issuance => option(option(bool())),
        Elements::IssuanceAsset | Elements::IssuanceToken => option(option(ExplicitAsset)),
        Elements::IssuanceEntropy => option(option(U256)),
        Elements::CalculateIssuanceEntropy => U256.into(),
        Elements::CalculateAsset
        | Elements::CalculateExplicitToken
        | Elements::CalculateConfidentialToken => ExplicitAsset.into(),
        /*
         * Transaction
         */
        Elements::TapleafVersion => U8.into(),
        Elements::CurrentIndex
        | Elements::NumInputs
        | Elements::NumOutputs
        | Elements::CurrentSequence
        | Elements::Version => U32.into(),
        Elements::ScriptCMR
        | Elements::CurrentScriptHash
        | Elements::CurrentScriptSigHash
        | Elements::CurrentIssuanceAssetProof
        | Elements::CurrentIssuanceTokenProof
        | Elements::GenesisBlockHash => U256.into(),
        Elements::InternalKey => Pubkey.into(),
        Elements::LockTime => Lock.into(),
        Elements::InputSequence => option(U32),
        Elements::OutputAsset => option(Asset1),
        Elements::OutputAmount => option(tuple([Asset1, Amount1])),
        Elements::OutputNonce => option(option(Nonce)),
        Elements::OutputScriptHash
        | Elements::OutputSurjectionProof
        | Elements::OutputRangeProof
        | Elements::CurrentPegin
        | Elements::CurrentAnnexHash
        | Elements::CurrentNewIssuanceContract
        | Elements::CurrentReissuanceEntropy
        | Elements::InputScriptHash
        | Elements::InputScriptSigHash
        | Elements::IssuanceAssetProof
        | Elements::IssuanceTokenProof
        | Elements::Tappath => option(U256),
        Elements::OutputNullDatum => option(option(either(tuple([U2, U256]), either(U1, U4)))),
        Elements::OutputIsFee => option(bool()),
        Elements::TotalFee => ExplicitAmount.into(),
        Elements::CurrentPrevOutpoint => Outpoint.into(),
        Elements::CurrentAsset => Asset1.into(),
        Elements::CurrentAmount => tuple([Asset1, Amount1]),
        Elements::CurrentReissuanceBlinding => option(ExplicitNonce),
        Elements::CurrentIssuanceAssetAmount => option(Amount1),
        Elements::CurrentIssuanceTokenAmount => option(TokenAmount1),
        Elements::InputPegin
        | Elements::InputAnnexHash
        | Elements::NewIssuanceContract
        | Elements::ReissuanceEntropy => option(option(U256)),
        Elements::InputPrevOutpoint => option(Outpoint),
        Elements::InputAsset => option(Asset1),
        Elements::InputAmount => option(tuple([Asset1, Amount1])),
        Elements::ReissuanceBlinding => option(option(ExplicitNonce)),
        Elements::IssuanceAssetAmount => option(option(Amount1)),
        Elements::IssuanceTokenAmount => option(option(TokenAmount1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::jet::Jet;

    #[rustfmt::skip]
    const ALL: [Elements; 460] = [
        Elements::Add16, Elements::Add32, Elements::Add64, Elements::Add8, Elements::All16, Elements::All32, Elements::All64, Elements::All8, Elements::And1, Elements::And16, Elements::And32, Elements::And64, Elements::And8, Elements::AnnexHash, Elements::AssetAmountHash, Elements::Bip0340Verify, Elements::BuildTapbranch, Elements::BuildTapleafSimplicity, Elements::CalculateAsset, Elements::CalculateConfidentialToken, Elements::CalculateExplicitToken, Elements::CalculateIssuanceEntropy, Elements::Ch1, Elements::Ch16, Elements::Ch32, Elements::Ch64, Elements::Ch8, Elements::CheckLockDistance, Elements::CheckLockDuration, Elements::CheckLockHeight, Elements::CheckLockTime, Elements::CheckSigVerify, Elements::Complement1, Elements::Complement16, Elements::Complement32, Elements::Complement64, Elements::Complement8, Elements::CurrentAmount, Elements::CurrentAnnexHash, Elements::CurrentAsset, Elements::CurrentIndex, Elements::CurrentIssuanceAssetAmount, Elements::CurrentIssuanceAssetProof, Elements::CurrentIssuanceTokenAmount, Elements::CurrentIssuanceTokenProof, Elements::CurrentNewIssuanceContract, Elements::CurrentPegin, Elements::CurrentPrevOutpoint, Elements::CurrentReissuanceBlinding, Elements::CurrentReissuanceEntropy, Elements::CurrentScriptHash, Elements::CurrentScriptSigHash, Elements::CurrentSequence, Elements::Decompress, Elements::Decrement16, Elements::Decrement32, Elements::Decrement64, Elements::Decrement8, Elements::DivMod16, Elements::DivMod32, Elements::DivMod64, Elements::DivMod8, Elements::Divide16, Elements::Divide32, Elements::Divide64, Elements::Divide8, Elements::Divides16, Elements::Divides32, Elements::Divides64, Elements::Divides8, Elements::Eq1, Elements::Eq16, Elements::Eq256, Elements::Eq32, Elements::Eq64, Elements::Eq8, Elements::FeAdd, Elements::FeInvert, Elements::FeIsOdd, Elements::FeIsZero, Elements::FeMultiply, Elements::FeMultiplyBeta, Elements::FeNegate, Elements::FeNormalize, Elements::FeSquare, Elements::FeSquareRoot, Elements::FullAdd16, Elements::FullAdd32, Elements::FullAdd64, Elements::FullAdd8, Elements::FullDecrement16, Elements::FullDecrement32, Elements::FullDecrement64, Elements::FullDecrement8, Elements::FullIncrement16, Elements::FullIncrement32, Elements::FullIncrement64, Elements::FullIncrement8, Elements::FullLeftShift16_1, Elements::FullLeftShift16_2, Elements::FullLeftShift16_4, Elements::FullLeftShift16_8, Elements::FullLeftShift32_1, Elements::FullLeftShift32_16, Elements::FullLeftShift32_2, Elements::FullLeftShift32_4, Elements::FullLeftShift32_8, Elements::FullLeftShift64_1, Elements::FullLeftShift64_16, Elements::FullLeftShift64_2, Elements::FullLeftShift64_32, Elements::FullLeftShift64_4, Elements::FullLeftShift64_8, Elements::FullLeftShift8_1, Elements::FullLeftShift8_2, Elements::FullLeftShift8_4, Elements::FullMultiply16, Elements::FullMultiply32, Elements::FullMultiply64, Elements::FullMultiply8, Elements::FullRightShift16_1, Elements::FullRightShift16_2, Elements::FullRightShift16_4, Elements::FullRightShift16_8, Elements::FullRightShift32_1, Elements::FullRightShift32_16, Elements::FullRightShift32_2, Elements::FullRightShift32_4, Elements::FullRightShift32_8, Elements::FullRightShift64_1, Elements::FullRightShift64_16, Elements::FullRightShift64_2, Elements::FullRightShift64_32, Elements::FullRightShift64_4, Elements::FullRightShift64_8, Elements::FullRightShift8_1, Elements::FullRightShift8_2, Elements::FullRightShift8_4, Elements::FullSubtract16, Elements::FullSubtract32, Elements::FullSubtract64, Elements::FullSubtract8, Elements::GeIsOnCurve, Elements::GeNegate, Elements::GejAdd, Elements::GejDouble, Elements::GejEquiv, Elements::GejGeAdd, Elements::GejGeAddEx, Elements::GejGeEquiv, Elements::GejInfinity, Elements::GejIsInfinity, Elements::GejIsOnCurve, Elements::GejNegate, Elements::GejNormalize, Elements::GejRescale, Elements::GejXEquiv, Elements::GejYIsOdd, Elements::Generate, Elements::GenesisBlockHash, Elements::High1, Elements::High16, Elements::High32, Elements::High64, Elements::High8, Elements::Increment16, Elements::Increment32, Elements::Increment64, Elements::Increment8, Elements::InputAmount, Elements::InputAmountsHash, Elements::InputAnnexHash, Elements::InputAnnexesHash, Elements::InputAsset, Elements::InputOutpointsHash, Elements::InputPegin, Elements::InputPrevOutpoint, Elements::InputScriptHash, Elements::InputScriptSigHash, Elements::InputScriptSigsHash, Elements::InputScriptsHash, Elements::InputSequence, Elements::InputSequencesHash, Elements::InputUtxosHash, Elements::InputsHash, Elements::InternalKey, Elements::IsOne16, Elements::IsOne32, Elements::IsOne64, Elements::IsOne8, Elements::IsZero16, Elements::IsZero32, Elements::IsZero64, Elements::IsZero8, Elements::Issuance, Elements::IssuanceAsset, Elements::IssuanceAssetAmount, Elements::IssuanceAssetAmountsHash, Elements::IssuanceAssetProof, Elements::IssuanceBlindingEntropyHash, Elements::IssuanceEntropy, Elements::IssuanceRangeProofsHash, Elements::IssuanceToken, Elements::IssuanceTokenAmount, Elements::IssuanceTokenAmountsHash, Elements::IssuanceTokenProof, Elements::IssuancesHash, Elements::Le16, Elements::Le32, Elements::Le64, Elements::Le8, Elements::LeftExtend16_32, Elements::LeftExtend16_64, Elements::LeftExtend1_16, Elements::LeftExtend1_32, Elements::LeftExtend1_64, Elements::LeftExtend1_8, Elements::LeftExtend32_64, Elements::LeftExtend8_16, Elements::LeftExtend8_32, Elements::LeftExtend8_64, Elements::LeftPadHigh16_32, Elements::LeftPadHigh16_64, Elements::LeftPadHigh1_16, Elements::LeftPadHigh1_32, Elements::LeftPadHigh1_64, Elements::LeftPadHigh1_8, Elements::LeftPadHigh32_64, Elements::LeftPadHigh8_16, Elements::LeftPadHigh8_32, Elements::LeftPadHigh8_64, Elements::LeftPadLow16_32, Elements::LeftPadLow16_64, Elements::LeftPadLow1_16, Elements::LeftPadLow1_32, Elements::LeftPadLow1_64, Elements::LeftPadLow1_8, Elements::LeftPadLow32_64, Elements::LeftPadLow8_16, Elements::LeftPadLow8_32, Elements::LeftPadLow8_64, Elements::LeftRotate16, Elements::LeftRotate32, Elements::LeftRotate64, Elements::LeftRotate8, Elements::LeftShift16, Elements::LeftShift32, Elements::LeftShift64, Elements::LeftShift8, Elements::LeftShiftWith16, Elements::LeftShiftWith32, Elements::LeftShiftWith64, Elements::LeftShiftWith8, Elements::Leftmost16_1, Elements::Leftmost16_2, Elements::Leftmost16_4, Elements::Leftmost16_8, Elements::Leftmost32_1, Elements::Leftmost32_16, Elements::Leftmost32_2, Elements::Leftmost32_4, Elements::Leftmost32_8, Elements::Leftmost64_1, Elements::Leftmost64_16, Elements::Leftmost64_2, Elements::Leftmost64_32, Elements::Leftmost64_4, Elements::Leftmost64_8, Elements::Leftmost8_1, Elements::Leftmost8_2, Elements::Leftmost8_4, Elements::LinearCombination1, Elements::LinearVerify1, Elements::LockTime, Elements::Low1, Elements::Low16, Elements::Low32, Elements::Low64, Elements::Low8, Elements::Lt16, Elements::Lt32, Elements::Lt64, Elements::Lt8, Elements::Maj1, Elements::Maj16, Elements::Maj32, Elements::Maj64, Elements::Maj8, Elements::Max16, Elements::Max32, Elements::Max64, Elements::Max8, Elements::Median16, Elements::Median32, Elements::Median64, Elements::Median8, Elements::Min16, Elements::Min32, Elements::Min64, Elements::Min8, Elements::Modulo16, Elements::Modulo32, Elements::Modulo64, Elements::Modulo8, Elements::Multiply16, Elements::Multiply32, Elements::Multiply64, Elements::Multiply8, Elements::Negate16, Elements::Negate32, Elements::Negate64, Elements::Negate8, Elements::NewIssuanceContract, Elements::NonceHash, Elements::NumInputs, Elements::NumOutputs, Elements::One16, Elements::One32, Elements::One64, Elements::One8, Elements::Or1, Elements::Or16, Elements::Or32, Elements::Or64, Elements::Or8, Elements::OutpointHash, Elements::OutputAmount, Elements::OutputAmountsHash, Elements::OutputAsset, Elements::OutputIsFee, Elements::OutputNonce, Elements::OutputNoncesHash, Elements::OutputNullDatum, Elements::OutputRangeProof, Elements::OutputRangeProofsHash, Elements::OutputScriptHash, Elements::OutputScriptsHash, Elements::OutputSurjectionProof, Elements::OutputSurjectionProofsHash, Elements::OutputsHash, Elements::ParseLock, Elements::ParseSequence, Elements::PointVerify1, Elements::ReissuanceBlinding, Elements::ReissuanceEntropy, Elements::RightExtend16_32, Elements::RightExtend16_64, Elements::RightExtend32_64, Elements::RightExtend8_16, Elements::RightExtend8_32, Elements::RightExtend8_64, Elements::RightPadHigh16_32, Elements::RightPadHigh16_64, Elements::RightPadHigh1_16, Elements::RightPadHigh1_32, Elements::RightPadHigh1_64, Elements::RightPadHigh1_8, Elements::RightPadHigh32_64, Elements::RightPadHigh8_16, Elements::RightPadHigh8_32, Elements::RightPadHigh8_64, Elements::RightPadLow16_32, Elements::RightPadLow16_64, Elements::RightPadLow1_16, Elements::RightPadLow1_32, Elements::RightPadLow1_64, Elements::RightPadLow1_8, Elements::RightPadLow32_64, Elements::RightPadLow8_16, Elements::RightPadLow8_32, Elements::RightPadLow8_64, Elements::RightRotate16, Elements::RightRotate32, Elements::RightRotate64, Elements::RightRotate8, Elements::RightShift16, Elements::RightShift32, Elements::RightShift64, Elements::RightShift8, Elements::RightShiftWith16, Elements::RightShiftWith32, Elements::RightShiftWith64, Elements::RightShiftWith8, Elements::Rightmost16_1, Elements::Rightmost16_2, Elements::Rightmost16_4, Elements::Rightmost16_8, Elements::Rightmost32_1, Elements::Rightmost32_16, Elements::Rightmost32_2, Elements::Rightmost32_4, Elements::Rightmost32_8, Elements::Rightmost64_1, Elements::Rightmost64_16, Elements::Rightmost64_2, Elements::Rightmost64_32, Elements::Rightmost64_4, Elements::Rightmost64_8, Elements::Rightmost8_1, Elements::Rightmost8_2, Elements::Rightmost8_4, Elements::ScalarAdd, Elements::ScalarInvert, Elements::ScalarIsZero, Elements::ScalarMultiply, Elements::ScalarMultiplyLambda, Elements::ScalarNegate, Elements::ScalarNormalize, Elements::ScalarSquare, Elements::Scale, Elements::ScriptCMR, Elements::Sha256Block, Elements::Sha256Ctx8Add1, Elements::Sha256Ctx8Add128, Elements::Sha256Ctx8Add16, Elements::Sha256Ctx8Add2, Elements::Sha256Ctx8Add256, Elements::Sha256Ctx8Add32, Elements::Sha256Ctx8Add4, Elements::Sha256Ctx8Add512, Elements::Sha256Ctx8Add64, Elements::Sha256Ctx8Add8, Elements::Sha256Ctx8AddBuffer511, Elements::Sha256Ctx8Finalize, Elements::Sha256Ctx8Init, Elements::Sha256Iv, Elements::SigAllHash, Elements::Some1, Elements::Some16, Elements::Some32, Elements::Some64, Elements::Some8, Elements::Subtract16, Elements::Subtract32, Elements::Subtract64, Elements::Subtract8, Elements::TapEnvHash, Elements::TapleafHash, Elements::TapleafVersion, Elements::Tappath, Elements::TappathHash, Elements::TotalFee, Elements::TxHash, Elements::TxIsFinal, Elements::TxLockDistance, Elements::TxLockDuration, Elements::TxLockHeight, Elements::TxLockTime, Elements::Verify, Elements::Version, Elements::Xor1, Elements::Xor16, Elements::Xor32, Elements::Xor64, Elements::Xor8, Elements::XorXor1, Elements::XorXor16, Elements::XorXor32, Elements::XorXor64, Elements::XorXor8
    ];

    #[test]
    fn compatible_source_type() {
        for jet in ALL {
            let resolved_ty = ResolvedType::tuple(
                source_type(jet)
                    .into_iter()
                    .map(|t| t.resolve_builtin().unwrap()),
            );
            let structural_ty = StructuralType::from(&resolved_ty);
            let simplicity_ty = jet.source_ty().to_final();

            println!("{jet}");
            assert_eq!(structural_ty.as_ref(), simplicity_ty.as_ref());
        }
    }

    #[test]
    fn compatible_target_type() {
        for jet in ALL {
            let resolved_ty = target_type(jet).resolve_builtin().unwrap();
            let structural_ty = StructuralType::from(&resolved_ty);
            let simplicity_ty = jet.target_ty().to_final();

            println!("{jet}");
            assert_eq!(structural_ty.as_ref(), simplicity_ty.as_ref());
        }
    }
}

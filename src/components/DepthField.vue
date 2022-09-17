
<script lang="ts" setup>
// import { reactive } from 'vue'
// do not use same name with ref
import type { FormInstance, FormRules } from 'element-plus'

const form = reactive({
    CoC: { id: 0, value: 0.013 }, // CIRCLE OF CONFUSION (COC)
    focus_length: "50.0", // 焦距
    aperture: "1.781797", // 光圈
    object_distance: 3, // 物距
    od_units: "1.0", // 单位(米)
})

const resultForm = ref({
    dofNear: "", // 景深近端距离(前景深)
    dofFar: "", // 景深远端距离(后景深)
    dofTotal: "", //  总景深
    dofFront: "",// 焦点前侧范围
    dofRear: "", // 焦点后侧范围
    hyperFocal: "", // 超焦距
    cocused: "" // 弥散圆直径
})

const isShowComputedResult = ref(false);

function doDepthOfField() {
    isShowComputedResult.value = true;
    console.log(form);
    // get input data from the form
    let distance = form.object_distance;
    let CoC = form.CoC.value;
    let aperture = parseFloat(form.aperture);
    let focal = parseFloat(form.focus_length);
    let units = parseFloat(form.od_units);
    console.log(distance, CoC, aperture, focal, units);
    // ensure that we have numbers in the variables
    let bCalcDistance: boolean;
    let unitsString: string;
    // check if distance input is OK
    if (isNaN(distance) || distance <= 0) bCalcDistance = false;
    else bCalcDistance = true; // 
    // calculate hyperfocal and near distance
    let hyperFocal = (focal * focal) / (aperture * CoC) + focal; // 超焦距
    let dofNear = 0.0; // 景深近端距离(前景深)
    let dofFar = 0.0; // 景深远端距离(后景深)
    let dofTotal = 0.0; //  总景深
    let dofNearPercent = 0.0;
    let dofFarPercent = 0.0;
    if (bCalcDistance) {
        distance = distance * 1000 * units; // convert to millimeters
        dofNear = ((hyperFocal - focal) * distance) / (hyperFocal + distance - (2 * focal));
        // Prevent 'divide by zero' when calculating far distance.
        if ((hyperFocal - distance) <= 0.00001) dofFar = 10000000.0; // set infinity at 10000m
        else dofFar = ((hyperFocal - focal) * distance) / (hyperFocal - distance);
        // Calculate percentage of DOF in front and behind the subject.
        dofNearPercent = (distance - dofNear) / (dofFar - dofNear) * 100.0;
        dofFarPercent = (dofFar - distance) / (dofFar - dofNear) * 100.0;
        // Convert depth of field numbers to proper units
        dofNear = dofNear / 1000.0 / units;
        dofFar = dofFar / 1000.0 / units;
        dofTotal = dofFar - dofNear;
        distance = distance / 1000.0 / units;
    }
    // convert hyperfocal distance to proper units
    hyperFocal = hyperFocal / 1000.0 / units;
    // set the units string
    if (units > 0.4) unitsString = " m";
    else if (units < 0.02) unitsString = " cm";
    else if (units < 0.1) unitsString = " in";
    else unitsString = " ft";
    // transfer values to the form
    let thresholdValue = 100;
    thresholdValue = hyperFocal < 10.0 ? 100 : 10;
    resultForm.value.hyperFocal = Math.round(hyperFocal * thresholdValue) / thresholdValue + unitsString
    thresholdValue = dofNear < 10.0 ? 100 : 10;
    resultForm.value.dofNear = Math.round(dofNear * thresholdValue) / thresholdValue + unitsString


    // if (bCalcDistance) {
    if (dofFar < 10000.0) {
        thresholdValue = dofFar < 10.0 ? 100 : 10;
        resultForm.value.dofFar = Math.round(dofFar * thresholdValue) / thresholdValue + unitsString
        thresholdValue = dofTotal < 10.0 ? 100 : 10;
        resultForm.value.dofTotal = Math.round(dofTotal * thresholdValue) / thresholdValue + unitsString
        resultForm.value.dofFront = Math.round((distance - dofNear) * thresholdValue) / thresholdValue + unitsString + "\t(" + Math.round(dofNearPercent) + "%)"
        resultForm.value.dofRear = Math.round((dofFar - distance) * thresholdValue) / thresholdValue + unitsString + "\t(" + Math.round(dofFarPercent) + "%)"

    } else {
        resultForm.value.dofFar = "无穷大";
        resultForm.value.dofTotal = "无穷大";
        // 焦点前侧范围
        resultForm.value.dofFront = Math.round((distance - dofNear) * 10) / 10 + unitsString;
        // 焦点后侧范围
        resultForm.value.dofRear = "无穷大";
    }
    // }
    resultForm.value.cocused = CoC + " mm" // 弥散圆直径 
}

const onSubmit = () => {
    console.log('submit!')
}


const cameraOptions = [
    {
        label: '通用',
        options: [{
            value: { id: 100, value: 0.015 },
            label: 'M4/3'
        },
        {
            value: { id: 101, value: 0.019 },
            label: 'APS-C'
        },
        {
            value: { id: 102, value: 0.029 },
            label: '全画幅'
        },
        {
            value: { id: 104, value: 0.018 },
            label: '佳能APS-C'
        }
        ]
    },
    {
        label: 'Blackmagic',
        options: [{
            value: { id: 0, value: 0.013 },
            label: 'Blackmagic Cinema Camera'
        },
        {
            value: { id: 1, value: 0.01 },
            label: 'Blackmagic Pocket Cinema Camera, Studio Camera HD'
        },
        {
            value: { id: 2, value: 0.01 },
            label: 'Blackmagic Studio Camera 4K, URSA Broadcast/HDMI'
        },
        {
            value: { id: 3, value: 0.017 },
            label: 'Blackmagic Production Camera 4K, URSA EF/PL'
        }]
    },
    {
        label: 'Canon',
        options: [{
            value: { id: 4, value: 0.019 },
            label: 'Canon Digital Rebel, XS, XSi, XT, XTi'
        },
        {
            value: { id: 7, value: 0.019 },
            label: 'Canon EOS 200D, 7D, 70D, 30D, 20Da, 20D, 10D'
        },
        {
            value: { id: 8, value: 0.019 },
            label: 'Canon EOS 60Da, 60D, 50D, 40D'
        },
        { value: { id: 9, value: 0.019 }, label: 'Canon EOS D60, D30' },
        {
            value: { id: 10, value: 0.024 },
            label: 'Canon EOS 1D, 1D Mark II, 1D Mark II N, 1D Mark III'
        },
        { value: { id: 11, value: 0.023 }, label: 'Canon EOS 1D Mark IV' },
        {
            value: { id: 12, value: 0.03 },
            label: 'Canon EOS 1Ds, 1Ds Mark I, 1DS Mark II, 1DS Mark III'
        },
        {
            value: { id: 13, value: 0.03 },
            label: 'Canon EOS R, 1D C, 1D X, 1D X Mark II'
        },
        { value: { id: 15, value: 0.03 }, label: 'Canon EOS 6D, 6D Mark II' },
        { value: { id: 16, value: 0.019 }, label: 'Canon EOS 7D, 7D Mark II' },
        {
            value: { id: 17, value: 0.019 },
            label: 'Canon EOS 800D, 760D, 750D, 700D, 650D, 600D, 550D, M3'
        }]
    },
    {
        label: 'Fujifilm',
        options: [{
            value: { id: 19, value: 0.019 },
            label: 'Fujifilm FinePix S5 Pro, S3 Pro, S2 Pro, S1 Pro, IS Pro'
        },
        { value: { id: 89, value: 0.038 }, label: 'Fujifilm GFX 50S, 50R' }]
    },
    {
        label: 'Hasselblad',
        options: [{
            value: { id: 21, value: 0.02 },
            label: 'Hasselblad Lunar'
        },
        {
            value: { id: 22, value: 0.036 },
            label: 'Hasselblad 503 CW, 503 CWD'
        },
        { value: { id: 23, value: 0.038 }, label: 'Hasselblad H5D-40, H4D-40' },
        {
            value: { id: 24, value: 0.042 },
            label: 'Hasselblad H3D II-31, H3D II-39, H3D II-50'
        },
        {
            value: { id: 25, value: 0.043 },
            label: 'Hasselblad H5D-50, H4D-50MS, H4D-50, H4D-200MS'
        },
        { value: { id: 26, value: 0.047 }, label: 'Hasselblad H4D-60' }]
    },
    {
        label: 'Konica Minolta',
        options: [{
            value: { id: 27, value: 0.02 },
            label: 'Konica Minolta DYNAX 5D, 7D'
        },
        {
            value: { id: 28, value: 0.02 },
            label: 'Konica Minolta Maxxum 5D, 7D'
        }]
    },
    {
        label: 'Leaf',
        options: [{
            value: { id: 29, value: 0.038 },
            label: 'Leaf Aptus II 8, Aptus II 6, Credo 40'
        },
        {
            value: { id: 30, value: 0.042 },
            label: 'Leaf Aptus II 7, AptusII 5'
        },
        { value: { id: 31, value: 0.046 }, label: 'Leaf Aptus II 10' },
        { value: { id: 32, value: 0.047 }, label: 'Leaf Credo 80, Credo 60' }]
    },
    {
        label: 'Leica',
        options: [{
            value: { id: 33, value: 0.011 },
            label: 'Leica V-Lux (Typ 114)'
        },
        { value: { id: 34, value: 0.015 }, label: 'Leica D-Lux (Typ 109)' },
        { value: { id: 35, value: 0.016 }, label: 'Leica Digilux 3' },
        { value: { id: 36, value: 0.019 }, label: 'Leica X (Typ 113)' },
        {
            value: { id: 37, value: 0.02 },
            label: 'Leica X2, X1, X-E (Typ 102), X-U (Typ 113)'
        },
        { value: { id: 38, value: 0.023 }, label: 'Leica M8, M8.2' },
        {
            value: { id: 39, value: 0.03 },
            label: 'Leica M9, M9 P, M-Monochrom, Q (Typ 116)'
        },
        {
            value: { id: 40, value: 0.038 },
            label: 'Leica S, S2, S (Typ 007), S-E (Typ 006)'
        }]
    },
    {
        label: 'Mamiya',
        options: [{
            value: { id: 41, value: 0.038 },
            label: 'Mamiya DM40 Digital Back'
        },
        { value: { id: 42, value: 0.042 }, label: 'Mamiya ZD Back, DM33' }]
    },
    {
        label: 'Nikon',
        options: [{
            value: { id: 43, value: 0.011 },
            label: 'Nikon 1 J1, 1 J2, 1 V1, 1 V2, 1 V3'
        },
        { value: { id: 44, value: 0.019 }, label: 'Nikon D3400, D3200, D3100' },
        {
            value: { id: 45, value: 0.02 },
            label: 'Nikon D90, D80, D70, D70s, D60, D50, D40, D40x'
        },
        {
            value: { id: 47, value: 0.02 },
            label: 'Nikon D500, D300, D300S, D200, D100'
        },
        {
            value: { id: 48, value: 0.03 },
            label: 'Nikon ℤ9, ℤ7, ℤ6, ℤ5, D850, D810, D810A, D800, D800E, D750, D700, D610, D600'
        },
        {
            value: { id: 49, value: 0.02 },
            label: 'Nikon D2X, D2Xs, D2H, D2Hs, D1H, D1X, D1'
        },
        {
            value: { id: 50, value: 0.03 },
            label: 'Nikon D4S, D4, D3X, D3S, Df, Z6, Z7'
        }]
    },
    {
        label: 'Olympus',
        options: [{
            value: { id: 51, value: 0.015 },
            label: 'Olympus OM-D E-M10, OM-D E-M5, OM-D E-M1'
        },
        {
            value: { id: 53, value: 0.015 },
            label: 'Olympus PEN E-PM1, E-PM2, F'
        },
        {
            value: { id: 54, value: 0.016 },
            label: 'Olympus PEN E-P2, E-P1, E-PL1s'
        },
        {
            value: { id: 56, value: 0.016 },
            label: 'Olympus E-620, E-600, E-520, E-510, E-500'
        }]
    },
    {
        label: 'Panasonic',
        options: [{
            value: { id: 57, value: 0.015 },
            label: 'Panasonic Lumix DMC-G7, DMC-G5, DMC-G3'
        },
        {
            value: { id: 58, value: 0.016 },
            label: 'Panasonic Lumix DMC-G10, DMC-G2, DMC-G1, DMC-GF1'
        },
        {
            value: { id: 61, value: 0.015 },
            label: 'Panasonic Lumix DMC-GX7, DMC-GX1, DMC-GM5'
        },
        {
            value: { id: 62, value: 0.016 },
            label: 'Panasonic Lumix DMC-L10, DMC-L1'
        },
        {
            value: { id: 88, value: 0.0136 },
            label: 'Panasonic Lumix DMC-LX100'
        }]
    },
    {
        label: 'Pentax',
        options: [{
            value: { id: 63, value: 0.02 },
            label: 'Pentax K200D, K110D, K100D, K100D Super'
        },
        { value: { id: 64, value: 0.02 }, label: 'Pentax K2000, K20D, K10D' },
        { value: { id: 66, value: 0.02 }, label: 'Pentax K-30, K-01' },
        {
            value: { id: 67, value: 0.02 },
            label: 'Pentax *ist D, DL, DL2, DS DS2, K-S1'
        },
        { value: { id: 68, value: 0.038 }, label: 'Pentax 645D, 645Z' },
        { value: { id: 87, value: 0.03 }, label: 'Pentax K-1, K-1 Mark II' }]
    },
    {
        label: 'Phase One',
        options: [{
            value: { id: 69, value: 0.038 },
            label: 'Phase One P 40+ Digital Back, IQ140 Digital Back'
        },
        {
            value: { id: 70, value: 0.043 },
            label: 'Phase One P 45+ Digital Back'
        },
        {
            value: { id: 71, value: 0.047 },
            label: 'Phase One P 65+ Digital Back'
        }]
    },
    {
        label: 'Samsung',
        options: [{
            value: { id: 73, value: 0.02 },
            label: 'Samsung GX-20, GX-10, GX-1L, GX-1S'
        },
        {
            value: { id: 74, value: 0.02 },
            label: 'Samsung NX1, NX20, NX11, NX10, NX5'
        },
        {
            value: { id: 75, value: 0.02 },
            label: 'Samsung NX1000, NX300, NX210, NX200, NX100'
        }]
    },
    {
        label: 'Sigma',
        options: [{
            value: { id: 76, value: 0.017 },
            label: 'Sigma DP1, DP1s, DP1X, DP2, DP2s, DP2X'
        },
        {
            value: { id: 77, value: 0.017 },
            label: 'Sigma SD15, SD14, SD10, SD9'
        },
        { value: { id: 78, value: 0.02 }, label: 'Sigma SD1, SD1 Merrill' }]
    },
    {
        label: 'Sony',
        options: [{
            value: { id: 80, value: 0.02 },
            label: 'Sony Alpha DSLR-A700, A580, A560, A500, A550, A450, A390'
        },
        { value: { id: 81, value: 0.03 }, label: 'Sony Alpha DSLR-A900, A850' },
        {
            value: { id: 82, value: 0.02 },
            label: 'Sony a6500, a6300, a6000, a5000, a3000'
        },
        {
            value: { id: 83, value: 0.02 },
            label: 'Sony a77, a77II, a65, a57, a55, a37, a35, a33'
        },
        {
            value: { id: 84, value: 0.03 },
            label: 'Sony a9, a7, a7 II, a7 III, a7R, a7R II, a7s, a99, a99 II'
        },
        { value: { id: 85, value: 0.03 }, label: 'Sony Cyber-shot DSC-RX1' },
        {
            value: { id: 86, value: 0.011 },
            label: 'Sony Cyber-shot DSC-RX10, RX10 II, RX10 III, RX10 IV'
        }]
    }]


function handleSelected(x: any) {
    console.log(x);
    console.log(form.CoC);
}
const validateNumber = (rule: any, value: any, callback: any) => {
    console.log(value);
    if (value === '') {
        callback(new Error('请输入'))
    } else {
        let regPos = /^[0-9]+(\.[0-9]+)?$/;
        if (regPos.test(form.object_distance.toString())) {
            callback();
            console.log(value);
        } else {
            callback(new Error('必须是数字'))
        }
    }
}
const ruleFormRef = ref<FormInstance>();
const rules = reactive<FormRules>({
    object_distance: [{ validator: validateNumber, trigger: 'change' }]
});
</script>

<template>

    <el-form :model="form" label-width="120px" status-icon ref="ruleFormRef" :rules="rules">
        <el-row>


            <el-form-item label="">
                <span>胶片格式，数码相机，或自定义弥散圆</span>
                <el-select v-model="form.CoC" default-first-option @change="handleSelected" value-key="id"
                    placeholder="please select your zone" filterable>
                    <el-option-group v-for="group in cameraOptions" :label="group.label">
                        <el-option v-for="cameras in group.options" :value="cameras.value" :label="cameras.label" />
                    </el-option-group>
                </el-select>
            </el-form-item>
        </el-row>
        <el-row>
            <el-col :span="11">
                <el-form-item>
                    <span>镜头实际焦距(mm)</span>
                    <el-select v-model="form.focus_length" filterable>
                        <el-option value="3">3</el-option>
                        <el-option value="3.6">3.6</el-option>
                        <el-option value="4">4</el-option>
                        <el-option value="4.3">4.3</el-option>
                        <el-option value="4.5">4.5</el-option>
                        <el-option value="4.6">4.6</el-option>
                        <el-option value="4.7">4.7</el-option>
                        <el-option value="4.75">4.75</el-option>
                        <el-option value="5.0">5</el-option>
                        <el-option value="5.2">5.2</el-option>
                        <el-option value="5.3">5.3</el-option>
                        <el-option value="5.4">5.4</el-option>
                        <el-option value="5.5">5.5</el-option>
                        <el-option value="5.6">5.6</el-option>
                        <el-option value="5.7">5.7</el-option>
                        <el-option value="5.8">5.8</el-option>
                        <el-option value="5.9">5.9</el-option>
                        <el-option value="6.0">6</el-option>
                        <el-option value="6.1">6.1</el-option>
                        <el-option value="6.2">6.2</el-option>
                        <el-option value="6.3">6.3</el-option>
                        <el-option value="6.4">6.4</el-option>
                        <el-option value="6.5">6.5</el-option>
                        <el-option value="6.6">6.6</el-option>
                        <el-option value="6.8">6.8</el-option>
                        <el-option value="6.9">6.9</el-option>
                        <el-option value="7.0">7</el-option>
                        <el-option value="7.1">7.1</el-option>
                        <el-option value="7.15">7.15</el-option>
                        <el-option value="7.2">7.2</el-option>
                        <el-option value="7.25">7.25</el-option>
                        <el-option value="7.3">7.3</el-option>
                        <el-option value="7.4">7.4</el-option>
                        <el-option value="7.5">7.5</el-option>
                        <el-option value="7.6">7.6</el-option>
                        <el-option value="7.7">7.7</el-option>
                        <el-option value="7.8">7.8</el-option>
                        <el-option value="8.0">8</el-option>
                        <el-option value="8.2">8.2</el-option>
                        <el-option value="8.3">8.3</el-option>
                        <el-option value="9.0">9</el-option>
                        <el-option value="9.2">9.2</el-option>
                        <el-option value="9.7">9.7</el-option>
                        <el-option value="9.9">9.9</el-option>
                        <el-option value="10.0">10</el-option>
                        <el-option value="10.5">10.5</el-option>
                        <el-option value="10.8">10.8</el-option>
                        <el-option value="11.0">11</el-option>
                        <el-option value="12.0">12</el-option>
                        <el-option value="12.5">12.5</el-option>
                        <el-option value="12.6">12.6</el-option>
                        <el-option value="13.0">13</el-option>
                        <el-option value="13.5">13.5</el-option>
                        <el-option value="14.0">14</el-option>
                        <el-option value="15.0">15</el-option>
                        <el-option value="15.5">15.5</el-option>
                        <el-option value="15.6">15.6</el-option>
                        <el-option value="16.0">16</el-option>
                        <el-option value="16.2">16.2</el-option>
                        <el-option value="16.8">16.8</el-option>
                        <el-option value="17.0">17</el-option>
                        <el-option value="17.1">17.1</el-option>
                        <el-option value="17.4">17.4</el-option>
                        <el-option value="18.0">18</el-option>
                        <el-option value="18.3">18.3</el-option>
                        <el-option value="18.6">18.6</el-option>
                        <el-option value="19.0">19</el-option>
                        <el-option value="19.2">19.2</el-option>
                        <el-option value="19.5">19.5</el-option>
                        <el-option value="19.8">19.8</el-option>
                        <el-option value="20.0">20</el-option>
                        <el-option value="20.3">20.3</el-option>
                        <el-option value="21.0">21</el-option>
                        <el-option value="21.3">21.3</el-option>
                        <el-option value="21.4">21.4</el-option>
                        <el-option value="21.9">21.9</el-option>
                        <el-option value="22.0">22</el-option>
                        <el-option value="22.2">22.2</el-option>
                        <el-option value="22.5">22.5</el-option>
                        <el-option value="22.8">22.8</el-option>
                        <el-option value="23.0">23</el-option>
                        <el-option value="23.1">23.1</el-option>
                        <el-option value="23.4">23.4</el-option>
                        <el-option value="23.5">23.5</el-option>
                        <el-option value="24.0">24</el-option>
                        <el-option value="24.9">24.9</el-option>
                        <el-option value="25.0">25</el-option>
                        <el-option value="25.8">25.8</el-option>
                        <el-option value="26.0">26</el-option>
                        <el-option value="26.4">26.4</el-option>
                        <el-option value="27.0">27</el-option>
                        <el-option value="28.0">28</el-option>
                        <el-option value="28.6">28.6</el-option>
                        <el-option value="28.8">28.8</el-option>
                        <el-option value="30.0">30</el-option>
                        <el-option value="31.0">31</el-option>
                        <el-option value="32.0">32</el-option>
                        <el-option value="33.0">33</el-option>
                        <el-option value="34.0">34</el-option>
                        <el-option value="35.0">35</el-option>
                        <el-option value="35.5">35.5</el-option>
                        <el-option value="36.0">36</el-option>
                        <el-option value="37.0">37</el-option>
                        <el-option value="37.5">37.5</el-option>
                        <el-option value="38.0">38</el-option>
                        <el-option value="39.0">39</el-option>
                        <el-option value="40.0">40</el-option>
                        <el-option value="41.0">41</el-option>
                        <el-option value="42.0">42</el-option>
                        <el-option value="43.0">43</el-option>
                        <el-option value="45.0">45</el-option>
                        <el-option value="46.0">46</el-option>
                        <el-option value="46.8">46.8</el-option>
                        <el-option value="47.0">47</el-option>
                        <el-option value="48.0">48</el-option>
                        <el-option value="48.5">48.5</el-option>
                        <el-option value="50.0">50</el-option>
                        <el-option value="50.8">50.8</el-option>
                        <el-option value="50.9">50.9</el-option>
                        <el-option value="51.0">51.0</el-option>
                        <el-option value="51.2">51.2</el-option>
                        <el-option value="51.6">51.6</el-option>
                        <el-option value="52.0">52</el-option>
                        <el-option value="53.0">53</el-option>
                        <el-option value="54.0">54</el-option>
                        <el-option value="55.0">55</el-option>
                        <el-option value="55.2">55.2</el-option>
                        <el-option value="56.0">56</el-option>
                        <el-option value="58.0">58</el-option>
                        <el-option value="59.0">59</el-option>
                        <el-option value="60.0">60</el-option>
                        <el-option value="61.0">61</el-option>
                        <el-option value="63.0">63</el-option>
                        <el-option value="65.0">65</el-option>
                        <el-option value="66.0">66</el-option>
                        <el-option value="69.0">69</el-option>
                        <el-option value="70.0">70</el-option>
                        <el-option value="72.0">72</el-option>
                        <el-option value="75.0">75</el-option>
                        <el-option value="76.0">76</el-option>
                        <el-option value="80.0">80</el-option>
                        <el-option value="85.0">85</el-option>
                        <el-option value="86.0">86</el-option>
                        <el-option value="90.0">90</el-option>
                        <el-option value="95.0">95</el-option>
                        <el-option value="100.0">100</el-option>
                        <el-option value="101.0">101</el-option>
                        <el-option value="102.0">102</el-option>
                        <el-option value="103.0">103</el-option>
                        <el-option value="104.0">104</el-option>
                        <el-option value="105.0">105</el-option>
                        <el-option value="106.0">106</el-option>
                        <el-option value="110.0">110</el-option>
                        <el-option value="111.0">111</el-option>
                        <el-option value="114.0">114</el-option>
                        <el-option value="115.0">115</el-option>
                        <el-option value="117.0">117</el-option>
                        <el-option value="120.0">120</el-option>
                        <el-option value="121.0">121</el-option>
                        <el-option value="123.0">123</el-option>
                        <el-option value="125.0">125</el-option>
                        <el-option value="127.0">127</el-option>
                        <el-option value="130.0">130</el-option>
                        <el-option value="135.0">135</el-option>
                        <el-option value="140.0">140</el-option>
                        <el-option value="145.0">145</el-option>
                        <el-option value="150.0">150</el-option>
                        <el-option value="152.0">152</el-option>
                        <el-option value="155.0">155</el-option>
                        <el-option value="160.0">160</el-option>
                        <el-option value="165.0">165</el-option>
                        <el-option value="168.0">168</el-option>
                        <el-option value="170.0">170</el-option>
                        <el-option value="180.0">180</el-option>
                        <el-option value="190.0">190</el-option>
                        <el-option value="200.0">200</el-option>
                        <el-option value="203.0">203</el-option>
                        <el-option value="205.0">205</el-option>
                        <el-option value="210.0">210</el-option>
                        <el-option value="215.0">215</el-option>
                        <el-option value="240.0">240</el-option>
                        <el-option value="250.0">250</el-option>
                        <el-option value="254.0">254</el-option>
                        <el-option value="255.0">255</el-option>
                        <el-option value="260.0">260</el-option>
                        <el-option value="270.0">270</el-option>
                        <el-option value="280.0">280</el-option>
                        <el-option value="300.0">300</el-option>
                        <el-option value="305.0">305</el-option>
                        <el-option value="320.0">320</el-option>
                        <el-option value="350.0">350</el-option>
                        <el-option value="355.0">355</el-option>
                        <el-option value="360.0">360</el-option>
                        <el-option value="375.0">375</el-option>
                        <el-option value="380.0">380</el-option>
                        <el-option value="400.0">400</el-option>
                        <el-option value="420.0">420</el-option>
                        <el-option value="450.0">450</el-option>
                        <el-option value="480.0">480</el-option>
                        <el-option value="500.0">500</el-option>
                        <el-option value="600.0">600</el-option>
                        <el-option value="610.0">610</el-option>
                        <el-option value="720.0">720</el-option>
                        <el-option value="800.0">800</el-option>
                        <el-option value="1000.0">1000</el-option>
                        <el-option value="1200.0">1200</el-option>
                    </el-select>
                </el-form-item>
            </el-col>
            <el-col :span="2" class="text-center">
                <!-- <span class="text-gray-500">-</span> -->
            </el-col>
            <el-col :span="11">
                <el-form-item>
                    <span>光圈大小</span>
                    <el-select v-model="form.aperture" default-first-option filterable>
                        <el-option value="0.95" label="f/0.95" />
                        <el-option value="1.0" label="f/1" />
                        <el-option value="1.122462" label="f/1.1" />
                        <el-option value="1.189207" label="f/1.2" />
                        <el-option value="1.259921" label="f/1.3" />
                        <el-option value="1.414214" label="f/1.4" />
                        <el-option value="1.587401" label="f/1.6" />
                        <el-option value="1.681793" label="f/1.7" />
                        <el-option value="1.781797" label="f/1.8" />
                        <el-option value="2.000000" label="f/2" />
                        <el-option value="2.244924" label="f/2.2" />
                        <el-option value="2.378414" label="f/2.4" />
                        <el-option value="2.519842" label="f/2.5" />
                        <el-option value="2.828427" label="f/2.8" />
                        <el-option value="3.174802" label="f/3.2" />
                        <el-option value="3.363586" label="f/3.4" />
                        <el-option value="3.563595" label="f/3.6" />
                        <el-option value="4.000000" label="f/4" />
                        <el-option value="4.489848" label="f/4.5" />
                        <el-option value="4.756828" label="f/4.8" />
                        <el-option value="5.039684" label="f/5" />
                        <el-option value="5.656854" label="f/5.6" />
                        <el-option value="6.349604" label="f/6.4" />
                        <el-option value="6.727171" label="f/6.7" />
                        <el-option value="7.127190" label="f/7.1" />
                        <el-option value="8.000000" label="f/8" />
                        <el-option value="8.979696" label="f/9" />
                        <el-option value="9.513657" label="f/9.5" />
                        <el-option value="10.07937" label="f/10" />
                        <el-option value="11.313708" label="f/11" />
                        <el-option value="12.699208" label="f/12.7" />
                        <el-option value="13.454343" label="f/13.5" />
                        <el-option value="14.254379" label="f/14.3" />
                        <el-option value="16.000000" label="f/16" />
                        <el-option value="17.959393" label="f/18" />
                        <el-option value="19.027314" label="f/19" />
                        <el-option value="20.158737" label="f/20" />
                        <el-option value="22.627417" label="f/22" />
                        <el-option value="25.398417" label="f/25" />
                        <el-option value="26.908685" label="f/27" />
                        <el-option value="28.508759" label="f/28" />
                        <el-option value="32" label="f/32" />
                        <el-option value="45.254834" label="f/45" />
                        <el-option value="64" label="f/64" />
                    </el-select>
                </el-form-item>
            </el-col>
        </el-row>
        <el-row>
            <el-col :span="11">
                <el-form-item prop="object_distance">
                    <el-row>
                        <span class="ml-3 w-35 text-gray-600 inline-flex items-center">拍摄距离</span>
                    </el-row>
                    <el-row>
                        <el-input v-model="form.object_distance" @change="" />
                    </el-row>
                </el-form-item>
            </el-col>
            <el-col :span="2" class="text-center">
            </el-col>
            <el-col :span="11">
                <el-form-item>
                    <span>距离单位</span>
                    <el-select v-model="form.od_units" placeholder="选择单位">
                        <el-option label="米(m)" value="1.0" />
                        <el-option label="厘米(cm)" value="0.01" />
                        <el-option label="英尺(feet)" value="0.3048" />
                        <el-option label="英寸(in)" value="0.0254" />
                    </el-select>
                </el-form-item>
            </el-col>
        </el-row>
        <el-form-item inline>
            <!-- <el-col :span="11">
            </el-col>
            
            <el-col :span="11">
                
            </el-col> -->
            <el-button plain @click="doDepthOfField" color="#783471">计算</el-button>

        </el-form-item>
    </el-form>

    <el-divider></el-divider>
    <!-- <div v-else> -->
    <el-descriptions v-if="isShowComputedResult" title="计算结果" direction="vertical" :column="3" size="default" border>
        <el-descriptions-item label="景深近端距离(前景深)">{{resultForm.dofNear}}</el-descriptions-item>
        <el-descriptions-item label="景深远端距离(后景深)">{{resultForm.dofFar}}</el-descriptions-item>
        <el-descriptions-item label="总景深">{{resultForm.dofTotal}}</el-descriptions-item>
        <el-descriptions-item label="焦点前侧范围">{{resultForm.dofFront}}</el-descriptions-item>
        <el-descriptions-item label="焦点后侧范围">{{resultForm.dofRear}}</el-descriptions-item>
        <el-descriptions-item label="超焦距">{{resultForm.hyperFocal}}</el-descriptions-item>
        <el-descriptions-item label="弥散圆直径">{{resultForm.cocused}}</el-descriptions-item>
        <el-descriptions-item label="说明" color="#890978"><template #label>
                <div class="cell-item">
                    
                    <el-tag class="ml-2" type="danger"><el-icon>
                        <i-ep-Notification />
                    </el-icon>说明</el-tag>
                </div>
            </template>景深值越小（浅），虚化效果越好</el-descriptions-item>
    </el-descriptions>

    <!-- </div> -->

    <!-- TODO 添加公式 -->

    <div>
        <span>
            其中：
            δ——容许弥散圆直径
            F——镜头的拍摄光圈值
            f——镜头焦距
            L——对焦距离
            ΔL1——前景深
            ΔL2——后景深
            ΔL——景深
            公式：
            前景深 ΔL1=FδL2/（f2+FδL）
            后景深 ΔL2=FδL2/（f2−FδL）
            景深 ΔL=ΔL1+ΔL2=（2f2FδL2）/（f4−F2δ2L2）"
        </span>
    </div>
</template>

<style lang="scss" scoped>
// 下拉框
.tk-select-button {
    width: 100%;
    height: 48px;
    padding: 0 16px;
    border-radius: 12px;
    font-size: 14px;
    font-weight: 500;
    line-height: 48px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border: #E6E8EC 2px solid;
    background-color: #FCFCFD;
    cursor: pointer;
    transition: border .2s;
}

.tk-select-button:hover {
    border: #23262F 2px solid;
}

.tk-select-button span {
    font-weight: 500;
    user-select: none;
}

// icon
.select-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    border: #E6E8EC 2px solid;
    transition: all .2s;
}

.select-icon.selectOpen {
    transform: rotate(180deg);
}

// 下拉框
.tk-select-dropdown {
    position: fixed;
    background-color: #FCFCFD;
}

.tk-select-dropdown ul {
    overflow: hidden;
    border-radius: 12px;
    border: #E6E8EC 2px solid;
    box-shadow: 0 4px 12px rgba(35, 38, 47, 0.1);
}

.select-enter-from,
.select-leave-to {
    opacity: 0;
    transform: scale(0.9);
}

.select-enter-active,
.select-leave-active {
    transform-origin: top center;
    transition: opacity .4s cubic-bezier(0.5, 0, 0, 1.25), transform .2s cubic-bezier(0.5, 0, 0, 1.25);
}

.tk-select-item.active {
    color: #3772FF;
    background-color: #F3F5F6;
    user-select: none;
}
</style>
    
  
  
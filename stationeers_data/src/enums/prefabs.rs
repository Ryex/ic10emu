use serde_derive::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    AsRefStr,
    EnumProperty,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize
)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[strum(use_phf)]
#[repr(i32)]
pub enum StationpediaPrefab {
    #[strum(serialize = "ItemKitGroundTelescope")]
    #[strum(props(name = "Kit (Telescope)", desc = "", value = "-2140672772"))]
    ItemKitGroundTelescope = -2140672772i32,
    #[strum(serialize = "StructureSmallSatelliteDish")]
    #[strum(
        props(
            name = "Small Satellite Dish",
            desc = "This small communications unit can be used to communicate with nearby trade vessels.\n\n        When connected to a <link=ThingStructureComputer><color=green>Computer</color></link> containing a <link=ThingMotherboardComms><color=green>Communications Motherboard</color></link> motherboard, a <link=ThingLandingpad_CenterPiece01><color=green>Landingpad Center</color></link>, and a <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link>, this allows Stationeers to contact traders. Adjust its horizontal and vertical attributes either directly or through logic.",
            value = "-2138748650"
        )
    )]
    StructureSmallSatelliteDish = -2138748650i32,
    #[strum(serialize = "StructureStairwellBackRight")]
    #[strum(props(name = "Stairwell (Back Right)", desc = "", value = "-2128896573"))]
    StructureStairwellBackRight = -2128896573i32,
    #[strum(serialize = "StructureBench2")]
    #[strum(props(name = "Bench (High Tech Style)", desc = "", value = "-2127086069"))]
    StructureBench2 = -2127086069i32,
    #[strum(serialize = "ItemLiquidPipeValve")]
    #[strum(
        props(
            name = "Kit (Liquid Pipe Valve)",
            desc = "This kit creates a <link=ThingStructureLiquidValve><color=green>Liquid Valve</color></link>.",
            value = "-2126113312"
        )
    )]
    ItemLiquidPipeValve = -2126113312i32,
    #[strum(serialize = "ItemDisposableBatteryCharger")]
    #[strum(
        props(
            name = "Disposable Battery Charger",
            desc = "Consumable battery the recharges your suit battery. If used on a HEM-Droid it will recharge the HEM-Droids internal battery.",
            value = "-2124435700"
        )
    )]
    ItemDisposableBatteryCharger = -2124435700i32,
    #[strum(serialize = "StructureBatterySmall")]
    #[strum(
        props(
            name = "Auxiliary Rocket Battery ",
            desc = "0.Empty\n1.Critical\n2.VeryLow\n3.Low\n4.Medium\n5.High\n6.Full",
            value = "-2123455080"
        )
    )]
    StructureBatterySmall = -2123455080i32,
    #[strum(serialize = "StructureLiquidPipeAnalyzer")]
    #[strum(props(name = "Liquid Pipe Analyzer", desc = "", value = "-2113838091"))]
    StructureLiquidPipeAnalyzer = -2113838091i32,
    #[strum(serialize = "ItemGasTankStorage")]
    #[strum(
        props(
            name = "Kit (Canister Storage)",
            desc = "This kit produces a <link=ThingItemGasTankStorage><color=green>Kit (Canister Storage)</color></link> for refilling a <link=ThingItemGasCanisterEmpty><color=green>Canister</color></link>.",
            value = "-2113012215"
        )
    )]
    ItemGasTankStorage = -2113012215i32,
    #[strum(serialize = "StructureFrameCorner")]
    #[strum(
        props(
            name = "Steel Frame (Corner)",
            desc = "More durable than the <link=ThingStructureFrameIron><color=green>Iron Frame</color></link>, steel frames also offer several variations for more complex lattice constructions. \nWith a little patience and maneuvering, the corner frame's Gothic-inspired silhouette allows the creation of ogival arches and even more ambitious architecture, although they are not airtight and cannot be built on.",
            value = "-2112390778"
        )
    )]
    StructureFrameCorner = -2112390778i32,
    #[strum(serialize = "ItemPotatoBaked")]
    #[strum(props(name = "Baked Potato", desc = "", value = "-2111886401"))]
    ItemPotatoBaked = -2111886401i32,
    #[strum(serialize = "ItemFlashingLight")]
    #[strum(props(name = "Kit (Flashing Light)", desc = "", value = "-2107840748"))]
    ItemFlashingLight = -2107840748i32,
    #[strum(serialize = "ItemLiquidPipeVolumePump")]
    #[strum(props(name = "Kit (Liquid Volume Pump)", desc = "", value = "-2106280569"))]
    ItemLiquidPipeVolumePump = -2106280569i32,
    #[strum(serialize = "StructureAirlock")]
    #[strum(
        props(
            name = "Airlock",
            desc = "The standard airlock is a powered portal that forms the main component of an airlock chamber. As long as the airlock is not locked, it can be manually opened using a crowbar.",
            value = "-2105052344"
        )
    )]
    StructureAirlock = -2105052344i32,
    #[strum(serialize = "ItemCannedCondensedMilk")]
    #[strum(
        props(
            name = "Canned Condensed Milk",
            desc = "Made in an <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link> or <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link>, using <link=ThingItemCookedCondensedMilk><color=green>Condensed Milk</color></link> and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link>, canned condensed milk is fairly high in nutrition, and does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "-2104175091"
        )
    )]
    ItemCannedCondensedMilk = -2104175091i32,
    #[strum(serialize = "ItemKitHydraulicPipeBender")]
    #[strum(
        props(name = "Kit (Hydraulic Pipe Bender)", desc = "", value = "-2098556089")
    )]
    ItemKitHydraulicPipeBender = -2098556089i32,
    #[strum(serialize = "ItemKitLogicMemory")]
    #[strum(props(name = "Kit (Logic Memory)", desc = "", value = "-2098214189"))]
    ItemKitLogicMemory = -2098214189i32,
    #[strum(serialize = "StructureInteriorDoorGlass")]
    #[strum(
        props(
            name = "Interior Door Glass",
            desc = "0.Operate\n1.Logic",
            value = "-2096421875"
        )
    )]
    StructureInteriorDoorGlass = -2096421875i32,
    #[strum(serialize = "StructureAirConditioner")]
    #[strum(
        props(
            name = "Air Conditioner",
            desc = "Built using the <link=ThingItemKitAtmospherics><color=green>Kit (Atmospherics)</color></link>, the <link=ExMin><color=#0080FFFF>ExMin-designed</color></link> air conditioner is used to raise or lower input gas temperature.\n\t  \nThe unit has three pipe connections: input, output, and waste. Gas fed into the input will be heated or cooled to reach the target temperature, while the opposite will happen to gas on the waste network.\n\nMultiple Efficiency Multipliers can effect the amount of energy the Air Conditioner uses, and these can be view on the unit's green Information Panel. As the temperature difference between input and waste increases, the Temperature Differential Efficiency Multiplier will decrease. If input or waste temperature is extremely hot or cold, the Operational Temperature Efficiency will decrease. If the input or waste pipe has approach low pressures, the Pressure Efficiency will decrease.\n\n<link=ThingStructurePipeRadiator><color=green>Pipe Convection Radiator</color></link>s may be useful in bringing extreme pipe temperatures back towards normal world temperatures. \n  \nFor more information on using the air conditioner, consult the <link=TemperatureControlPage><color=#0080FFFF>temperature control</color></link> Guides page.",
            value = "-2087593337"
        )
    )]
    StructureAirConditioner = -2087593337i32,
    #[strum(serialize = "StructureRocketMiner")]
    #[strum(
        props(
            name = "Rocket Miner",
            desc = "Gathers available resources at the rocket's current space location.",
            value = "-2087223687"
        )
    )]
    StructureRocketMiner = -2087223687i32,
    #[strum(serialize = "DynamicGPR")]
    #[strum(
        props(
            name = "<N:EN:DynamicGPR>",
            desc = "<N:EN:DynamicGPR>",
            value = "-2085885850"
        )
    )]
    DynamicGpr = -2085885850i32,
    #[strum(serialize = "UniformCommander")]
    #[strum(props(name = "Uniform Commander", desc = "", value = "-2083426457"))]
    UniformCommander = -2083426457i32,
    #[strum(serialize = "StructureWindTurbine")]
    #[strum(
        props(
            name = "Wind Turbine",
            desc = "The Stationeers wind turbine was first designed by Norsec atmospheric engineers, looking to create a wind-driven power generation system that would operate even on exceedingly low atmosphere worlds. The ultra-light blades respond to exceedingly low atmospheric densities, while being strong enough to function even under huge strain in much more demanding environments.\nWhile the wind turbine is optimized to produce power (up to 500W) even on low atmosphere worlds, it performs best in denser environments. Output varies with wind speed and, during storms, may increase dramatically (up to 10,000W), so be careful to design your power networks with that in mind.",
            value = "-2082355173"
        )
    )]
    StructureWindTurbine = -2082355173i32,
    #[strum(serialize = "StructureInsulatedPipeTJunction")]
    #[strum(
        props(
            name = "Insulated Pipe (T Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "-2076086215"
        )
    )]
    StructureInsulatedPipeTJunction = -2076086215i32,
    #[strum(serialize = "ItemPureIcePollutedWater")]
    #[strum(
        props(
            name = "Pure Ice Polluted Water",
            desc = "A frozen chunk of <link=GasPollutedWater><color=#44AD83>Polluted Water</color></link>",
            value = "-2073202179"
        )
    )]
    ItemPureIcePollutedWater = -2073202179i32,
    #[strum(serialize = "RailingIndustrial02")]
    #[strum(
        props(name = "Railing Industrial (Type 2)", desc = "", value = "-2072792175")
    )]
    RailingIndustrial02 = -2072792175i32,
    #[strum(serialize = "StructurePipeInsulatedLiquidCrossJunction")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (Cross Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "-2068497073"
        )
    )]
    StructurePipeInsulatedLiquidCrossJunction = -2068497073i32,
    #[strum(serialize = "ItemWeldingTorch")]
    #[strum(
        props(
            name = "Welding Torch",
            desc = "Stored in the standard issue <link=Stationeers><color=#0080FFFF>Stationeers</color></link> <link=ThingItemToolBelt><color=green>Tool Belt</color></link>, the Arlite welding torch is used to construct a range of essential structures.\nAn upgraded version of the classic 'Zairo' model first manufactured by <link=ExMin><color=#0080FFFF>ExMin</color></link> for modular space habitat assembly, the Arlite is powered by a single <link=ThingItemGasCanisterFuel><color=green>Canister (Fuel)</color></link> and designed to function equally well in deep space and deep gravity wells.",
            value = "-2066892079"
        )
    )]
    ItemWeldingTorch = -2066892079i32,
    #[strum(serialize = "StructurePictureFrameThickMountPortraitSmall")]
    #[strum(
        props(
            name = "Picture Frame Thick Mount Portrait Small",
            desc = "",
            value = "-2066653089"
        )
    )]
    StructurePictureFrameThickMountPortraitSmall = -2066653089i32,
    #[strum(serialize = "Landingpad_DataConnectionPiece")]
    #[strum(
        props(
            name = "Landingpad Data And Power",
            desc = "Provides power to the landing pad. The data port must be connected to the data port of a computer with a communications motherboard for a trader to be called in to land.",
            value = "-2066405918"
        )
    )]
    LandingpadDataConnectionPiece = -2066405918i32,
    #[strum(serialize = "ItemKitPictureFrame")]
    #[strum(props(name = "Kit Picture Frame", desc = "", value = "-2062364768"))]
    ItemKitPictureFrame = -2062364768i32,
    #[strum(serialize = "ItemMKIIArcWelder")]
    #[strum(props(name = "Mk II Arc Welder", desc = "", value = "-2061979347"))]
    ItemMkiiArcWelder = -2061979347i32,
    #[strum(serialize = "StructureCompositeWindow")]
    #[strum(
        props(
            name = "Composite Window",
            desc = "Air-tight and resistant to extreme temperatures, composite walls come in several charming, near identical varieties - reflecting their designer's focus on form over function.",
            value = "-2060571986"
        )
    )]
    StructureCompositeWindow = -2060571986i32,
    #[strum(serialize = "ItemEmergencyDrill")]
    #[strum(props(name = "Emergency Drill", desc = "", value = "-2052458905"))]
    ItemEmergencyDrill = -2052458905i32,
    #[strum(serialize = "Rover_MkI")]
    #[strum(
        props(
            name = "Rover MkI",
            desc = "A distant cousin of the jeep, the Mk I {<link=Sinotai><color=#0080FFFF>Sinotai</color></link> electric rover is one of the most simple and durable light vehicles in the known universe. Able to carry two passengers and cargo such as the <link=ThingDynamicGasCanisterAir><color=green>Portable Gas Tank (Air)</color></link> or <link=ThingCrate><color=green><N:EN:Crate></color></link>, it is powered by up to three batteries, accepting everything including <link=ThingItemBatteryCellNuclear><color=green>Battery Cell (Nuclear)</color></link>.\nA quad-array of hub-mounted electric engines propels the reinforced aluminium frame over most terrain and modest obstacles. While the Mk I is designed for stability in low-horizontality circumstances, if it rolls, try using your <link=ThingItemCrowbar><color=green>Crowbar</color></link> to put it right way up.Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "-2049946335"
        )
    )]
    RoverMkI = -2049946335i32,
    #[strum(serialize = "StructureSolarPanel")]
    #[strum(
        props(
            name = "Solar Panel",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai's</color></link> standard solar panels are used for generating power from sunlight. They can be connected to <link=LogicPage><color=#0080FFFF>Logic</color></link> systems, in order to track sunlight, but their reduced during storms and when damaged. You can repair these using some trusty <link=ThingItemDuctTape><color=green>Duct Tape</color></link>.",
            value = "-2045627372"
        )
    )]
    StructureSolarPanel = -2045627372i32,
    #[strum(serialize = "CircuitboardShipDisplay")]
    #[strum(
        props(
            name = "Ship Display",
            desc = "When the original <link=Stationeers><color=#0080FFFF>Stationeer</color></link> Handbook collapsed under its own weight into a singularity, certain information was irretrievably lost. Amongst this mysterious corpus of knowledge is the exact purpose of the ship display board.",
            value = "-2044446819"
        )
    )]
    CircuitboardShipDisplay = -2044446819i32,
    #[strum(serialize = "StructureBench")]
    #[strum(
        props(
            name = "Powered Bench",
            desc = "When it's time to sit, nothing supports you like a bench. This bench is powered, so you can use appliances like the <link=ThingApplianceMicrowave><color=green>Microwave</color></link>.",
            value = "-2042448192"
        )
    )]
    StructureBench = -2042448192i32,
    #[strum(serialize = "StructurePictureFrameThickLandscapeSmall")]
    #[strum(
        props(
            name = "Picture Frame Thick Landscape Small",
            desc = "",
            value = "-2041566697"
        )
    )]
    StructurePictureFrameThickLandscapeSmall = -2041566697i32,
    #[strum(serialize = "ItemKitLargeSatelliteDish")]
    #[strum(
        props(name = "Kit (Large Satellite Dish)", desc = "", value = "-2039971217")
    )]
    ItemKitLargeSatelliteDish = -2039971217i32,
    #[strum(serialize = "ItemKitMusicMachines")]
    #[strum(props(name = "Kit (Music Machines)", desc = "", value = "-2038889137"))]
    ItemKitMusicMachines = -2038889137i32,
    #[strum(serialize = "ItemStelliteGlassSheets")]
    #[strum(
        props(
            name = "Stellite Glass Sheets",
            desc = "A stronger glass substitute.",
            value = "-2038663432"
        )
    )]
    ItemStelliteGlassSheets = -2038663432i32,
    #[strum(serialize = "ItemKitVendingMachine")]
    #[strum(props(name = "Kit (Vending Machine)", desc = "", value = "-2038384332"))]
    ItemKitVendingMachine = -2038384332i32,
    #[strum(serialize = "StructurePumpedLiquidEngine")]
    #[strum(
        props(
            name = "Pumped Liquid Engine",
            desc = "Liquid propellants bring greater efficiencies with Pumped Liquid Engine. Two inputs are provided so Stationeers can seperate their fuels, the Setting variable controls the mixing ratio of the inputs. The engine is designed to run on <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link> and <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link>, some Stationeers have reported excessive thrust values by switching to <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link>",
            value = "-2031440019"
        )
    )]
    StructurePumpedLiquidEngine = -2031440019i32,
    #[strum(serialize = "StructurePictureFrameThinLandscapeSmall")]
    #[strum(
        props(
            name = "Picture Frame Thin Landscape Small",
            desc = "",
            value = "-2024250974"
        )
    )]
    StructurePictureFrameThinLandscapeSmall = -2024250974i32,
    #[strum(serialize = "StructureStacker")]
    #[strum(
        props(
            name = "Stacker",
            desc = "A stacker is an important part of any automated chute network. The <link=Xigo><color=#0080FFFF>Xigo</color></link> ProKompile can be set manually or via logic, to make sure items passing through the stacker are maximized for your storage needs.\nThe ProKompile can stack a wide variety of things such as <link=IngotPage><color=#0080FFFF>ingots</color></link>, as well as splitting stacks into appropriate sizes as needed.",
            value = "-2020231820"
        )
    )]
    StructureStacker = -2020231820i32,
    #[strum(serialize = "ItemMKIIScrewdriver")]
    #[strum(
        props(
            name = "Mk II Screwdriver",
            desc = "This standard issue frictional adherence adjustor is a top of the line, bi-rotational model with a columnated uni-grip. It's definitely not just a screwdriver. Use it for construction and deconstruction of certain kits, and setting values on <link=LogicUnitPage><color=#0080FFFF>logic</color></link> units. The MK II is more resistant to temperature and pressure.",
            value = "-2015613246"
        )
    )]
    ItemMkiiScrewdriver = -2015613246i32,
    #[strum(serialize = "StructurePressurePlateLarge")]
    #[strum(props(name = "Trigger Plate (Large)", desc = "", value = "-2008706143"))]
    StructurePressurePlateLarge = -2008706143i32,
    #[strum(serialize = "StructurePipeLiquidCrossJunction5")]
    #[strum(
        props(
            name = "Liquid Pipe (5-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidCrossJunction5><color=green>Insulated Liquid Pipe (5-Way Junction)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-2006384159"
        )
    )]
    StructurePipeLiquidCrossJunction5 = -2006384159i32,
    #[strum(serialize = "ItemGasFilterWater")]
    #[strum(
        props(
            name = "Filter (Water)",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai</color></link> filters are used to capture various gases, which can be disposed of, or used elsewhere. <link=GasWater><color=#44AD83>Water</color></link> can be collected by filtering smelted <link=ThingItemIce><color=green>Ice (Water)</color></link>",
            value = "-1993197973"
        )
    )]
    ItemGasFilterWater = -1993197973i32,
    #[strum(serialize = "SpaceShuttle")]
    #[strum(
        props(
            name = "Space Shuttle",
            desc = "An antiquated <link=Sinotai><color=#0080FFFF>Sinotai</color></link> transport craft, long since decommissioned.",
            value = "-1991297271"
        )
    )]
    SpaceShuttle = -1991297271i32,
    #[strum(serialize = "SeedBag_Fern")]
    #[strum(
        props(
            name = "Fern Seeds",
            desc = "Grow a <link=ThingItemFern><color=green>Fern</color></link>.",
            value = "-1990600883"
        )
    )]
    SeedBagFern = -1990600883i32,
    #[strum(serialize = "ItemSecurityCamera")]
    #[strum(
        props(
            name = "Security Camera",
            desc = "Security cameras can be paired with a <link=ThingStructureMotionSensor><color=green>Motion Sensor</color></link>, then connected to a <link=ThingStructureConsole><color=green>Console</color></link> fitted with a <link=ThingCircuitboardCameraDisplay><color=green>Camera Display</color></link> for that 'always watched' feeling.",
            value = "-1981101032"
        )
    )]
    ItemSecurityCamera = -1981101032i32,
    #[strum(serialize = "CardboardBox")]
    #[strum(props(name = "Cardboard Box", desc = "", value = "-1976947556"))]
    CardboardBox = -1976947556i32,
    #[strum(serialize = "ItemSoundCartridgeSynth")]
    #[strum(props(name = "Sound Cartridge Synth", desc = "", value = "-1971419310"))]
    ItemSoundCartridgeSynth = -1971419310i32,
    #[strum(serialize = "StructureCornerLocker")]
    #[strum(props(name = "Corner Locker", desc = "", value = "-1968255729"))]
    StructureCornerLocker = -1968255729i32,
    #[strum(serialize = "StructureInsulatedPipeCorner")]
    #[strum(
        props(
            name = "Insulated Pipe (Corner)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "-1967711059"
        )
    )]
    StructureInsulatedPipeCorner = -1967711059i32,
    #[strum(serialize = "StructureWallArchCornerSquare")]
    #[strum(props(name = "Wall (Arch Corner Square)", desc = "", value = "-1963016580"))]
    StructureWallArchCornerSquare = -1963016580i32,
    #[strum(serialize = "StructureControlChair")]
    #[strum(
        props(
            name = "Control Chair",
            desc = "Once, these chairs were the heart of space-going behemoths. Now, they're items of nostalgia built only by a handful of Stationeers with a sense of history. In other words, kitsch.",
            value = "-1961153710"
        )
    )]
    StructureControlChair = -1961153710i32,
    #[strum(serialize = "PortableComposter")]
    #[strum(
        props(
            name = "Portable Composter",
            desc = "A simple composting device, the basic composter creates <link=ThingFertilizer><color=green>Fertilizer</color></link> out of organic matter. It accepts <link=OrganicPage><color=#0080FFFF>food</color></link>, <link=ThingDecayedFood><color=green>Decayed Food</color></link> or <link=ThingItemBiomass><color=green>Biomass</color></link>. It requires a full <link=ThingItemLiquidCanisterEmpty><color=green>Liquid Canister</color></link> and a battery to operate, accelerating the natural composting process.\nWhen processing, it releases nitrogen and volatiles, as well a small amount of heat.\n\n<size=120%><b>Compost composition</b></size>\nFertilizer is produced at a 1:3 ratio of fertilizer to ingredients. The fertilizer's effects on plants will vary depending on the respective proportions of its ingredients.\n\n- <link=OrganicPage><color=#0080FFFF>food</color></link> increases PLANT YIELD up to two times\n- <link=ThingDecayedFood><color=green>Decayed Food</color></link> increases plant GROWTH SPEED up to two times\n- <link=ThingItemBiomass><color=green>Biomass</color></link> increases the NUMBER OF GROWTH CYCLES the fertilizer lasts for",
            value = "-1958705204"
        )
    )]
    PortableComposter = -1958705204i32,
    #[strum(serialize = "CartridgeGPS")]
    #[strum(props(name = "GPS", desc = "", value = "-1957063345"))]
    CartridgeGps = -1957063345i32,
    #[strum(serialize = "StructureConsoleLED1x3")]
    #[strum(
        props(
            name = "LED Display (Large)",
            desc = "0.Default\n1.Percent\n2.Power",
            value = "-1949054743"
        )
    )]
    StructureConsoleLed1X3 = -1949054743i32,
    #[strum(serialize = "ItemDuctTape")]
    #[strum(
        props(
            name = "Duct Tape",
            desc = "In the distant past, one of Earth's great champions taught a generation of 'Fix-It People' that duct tape was the answer to any problem. <link=Stationeers><color=#0080FFFF>Stationeers</color></link> have demonstrated that this is truth holds strong, so long as the problem is a damaged <link=ThingItemEvaSuit><color=green>Eva Suit</color></link>, <link=ThingItemJetpackBasic><color=green>Jetpack Basic</color></link>, <link=ThingItemSpaceHelmet><color=green>Space Helmet</color></link>, or even a <link=ThingStructureSolarPanel><color=green>Solar Panel</color></link>.\nTo use on yourself: put duct tape in your active hand, hold RIGHT MOUSE BUTTON to automatically repair damage.",
            value = "-1943134693"
        )
    )]
    ItemDuctTape = -1943134693i32,
    #[strum(serialize = "DynamicLiquidCanisterEmpty")]
    #[strum(
        props(
            name = "Portable Liquid Tank",
            desc = "This portable tank stores liquid, and liquid only. You can bolt one to a <link=ThingItemTankConnectorLiquid><color=green>Kit (Liquid Tank Connector)</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link>, then connect it to a pipe network to refill it. You can refill a <link=ThingItemGasCanisterWater><color=green>Liquid Canister (Water)</color></link> by attaching it to the tank's striped section. Or you could use a <link=ThingItemWrench><color=green>Wrench</color></link> to attach it to a rocket and take it somewhere distant and dry, then feel good about yourself.",
            value = "-1939209112"
        )
    )]
    DynamicLiquidCanisterEmpty = -1939209112i32,
    #[strum(serialize = "ItemKitDeepMiner")]
    #[strum(props(name = "Kit (Deep Miner)", desc = "", value = "-1935075707"))]
    ItemKitDeepMiner = -1935075707i32,
    #[strum(serialize = "ItemKitAutomatedOven")]
    #[strum(props(name = "Kit (Automated Oven)", desc = "", value = "-1931958659"))]
    ItemKitAutomatedOven = -1931958659i32,
    #[strum(serialize = "MothershipCore")]
    #[strum(
        props(
            name = "Mothership Core",
            desc = "A relic of from an earlier era of space ambition, <link=Sinotai><color=#0080FFFF>Sinotai's</color></link> mothership cores formed the central element of a generation's space-going creations. While Sinotai's pivot to smaller, modular craft upset some purists, motherships continue to be built and maintained by dedicated enthusiasts.",
            value = "-1930442922"
        )
    )]
    MothershipCore = -1930442922i32,
    #[strum(serialize = "ItemKitSolarPanel")]
    #[strum(props(name = "Kit (Solar Panel)", desc = "", value = "-1924492105"))]
    ItemKitSolarPanel = -1924492105i32,
    #[strum(serialize = "CircuitboardPowerControl")]
    #[strum(
        props(
            name = "Power Control",
            desc = "Under distant suns and demanding environments, <link=Stationeers><color=#0080FFFF>Stationeer</color></link> systems need to balance reliability, resilience and versatility. The power control board allows remote enabling and disabling of selected devices, disconnecting manual operation. \n      \nThe circuit board has two modes: 'Link' switches all devices on or off; 'Toggle' switches each device to their alternate state. ",
            value = "-1923778429"
        )
    )]
    CircuitboardPowerControl = -1923778429i32,
    #[strum(serialize = "SeedBag_Tomato")]
    #[strum(
        props(
            name = "Tomato Seeds",
            desc = "Grow a <link=ThingItemTomato><color=green>Tomato</color></link>.",
            value = "-1922066841"
        )
    )]
    SeedBagTomato = -1922066841i32,
    #[strum(serialize = "StructureChuteUmbilicalFemale")]
    #[strum(props(name = "Umbilical Socket (Chute)", desc = "", value = "-1918892177"))]
    StructureChuteUmbilicalFemale = -1918892177i32,
    #[strum(serialize = "StructureMediumConvectionRadiator")]
    #[strum(
        props(
            name = "Medium Convection Radiator",
            desc = "A stand-alone radiator unit optimized for exchanging heat with its surrounding atmosphere.",
            value = "-1918215845"
        )
    )]
    StructureMediumConvectionRadiator = -1918215845i32,
    #[strum(serialize = "ItemGasFilterVolatilesInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Volatiles)",
            desc = "A filter that selectively targets <link=GasVolatiles><color=#44AD83>Volatiles</color></link>. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-1916176068"
        )
    )]
    ItemGasFilterVolatilesInfinite = -1916176068i32,
    #[strum(serialize = "MotherboardSorter")]
    #[strum(
        props(
            name = "Sorter Motherboard",
            desc = "Motherboards are connected to <link=ThingStructureComputer><color=green>Computer</color></link>s to perform various technical functions.\nThe <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> K-cops 10-10 sorter motherboard permits <link=Stationeers><color=#0080FFFF>Stationeers</color></link> to control which items a <link=ThingStructureSorter><color=green>Sorter</color></link> does, and does not, permit to pass.",
            value = "-1908268220"
        )
    )]
    MotherboardSorter = -1908268220i32,
    #[strum(serialize = "ItemSoundCartridgeDrums")]
    #[strum(props(name = "Sound Cartridge Drums", desc = "", value = "-1901500508"))]
    ItemSoundCartridgeDrums = -1901500508i32,
    #[strum(serialize = "StructureFairingTypeA3")]
    #[strum(props(name = "Fairing (Type A3)", desc = "", value = "-1900541738"))]
    StructureFairingTypeA3 = -1900541738i32,
    #[strum(serialize = "RailingElegant02")]
    #[strum(props(name = "Railing Elegant (Type 2)", desc = "", value = "-1898247915"))]
    RailingElegant02 = -1898247915i32,
    #[strum(serialize = "ItemStelliteIngot")]
    #[strum(props(name = "Ingot (Stellite)", desc = "", value = "-1897868623"))]
    ItemStelliteIngot = -1897868623i32,
    #[strum(serialize = "StructureSmallTableBacklessSingle")]
    #[strum(
        props(name = "Small (Table Backless Single)", desc = "", value = "-1897221677")
    )]
    StructureSmallTableBacklessSingle = -1897221677i32,
    #[strum(serialize = "StructureHydraulicPipeBender")]
    #[strum(
        props(
            name = "Hydraulic Pipe Bender",
            desc = "A go-to tool for all your atmospheric and plumbing needs, the <link=ExMin><color=#0080FFFF>ExMin</color></link> Atmoprinter will create everything from pipes, pumps and tanks, to vents and filters, ensuring your survival in any environment. Upgrade the Atmoprinter using a <link=ThingPipeBenderMod><color=green>Pipe Bender Mod</color></link> for additional recipes and faster processing speeds.",
            value = "-1888248335"
        )
    )]
    StructureHydraulicPipeBender = -1888248335i32,
    #[strum(serialize = "ItemWrench")]
    #[strum(
        props(
            name = "Wrench",
            desc = "One of humanity's enduring contributions to the cosmos, the wrench represents the essence of our species. A simple, effective and spiritually barren tool, use it to build and deconstruct a variety of <link=StructurePage><color=#0080FFFF>structures</color></link>",
            value = "-1886261558"
        )
    )]
    ItemWrench = -1886261558i32,
    #[strum(serialize = "SeedBag_SugarCane")]
    #[strum(props(name = "Sugarcane Seeds", desc = "", value = "-1884103228"))]
    SeedBagSugarCane = -1884103228i32,
    #[strum(serialize = "ItemSoundCartridgeBass")]
    #[strum(props(name = "Sound Cartridge Bass", desc = "", value = "-1883441704"))]
    ItemSoundCartridgeBass = -1883441704i32,
    #[strum(serialize = "ItemSprayCanGreen")]
    #[strum(
        props(
            name = "Spray Paint (Green)",
            desc = "Green is the color of life, and longing. Paradoxically, it's also the color of envy, and tolerance. It denotes sickness, youth, and wealth. But really, it's just what light does at around 500 billionths of a meter.",
            value = "-1880941852"
        )
    )]
    ItemSprayCanGreen = -1880941852i32,
    #[strum(serialize = "StructurePipeCrossJunction5")]
    #[strum(
        props(
            name = "Pipe (5-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCrossJunction5><color=green>Insulated Pipe (5-Way Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-1877193979"
        )
    )]
    StructurePipeCrossJunction5 = -1877193979i32,
    #[strum(serialize = "StructureSDBHopper")]
    #[strum(props(name = "SDB Hopper", desc = "", value = "-1875856925"))]
    StructureSdbHopper = -1875856925i32,
    #[strum(serialize = "ItemMKIIMiningDrill")]
    #[strum(
        props(
            name = "Mk II Mining Drill",
            desc = "The handheld 'Topo' tri-cone rotary mining drill was made for one thing: quick digging. Modeled on a classic <link=Recurso><color=#0080FFFF>Recurso</color></link> zero-g design, it functions equally well in vacuum and atmosphere, with cemented carbide bits to increase resilience and bearing life, and reduce spalling. As Jenk Murtons once said, 'The Topo don't stopo.' The MK II is more resistant to temperature and pressure.",
            value = "-1875271296"
        )
    )]
    ItemMkiiMiningDrill = -1875271296i32,
    #[strum(serialize = "Landingpad_TaxiPieceCorner")]
    #[strum(props(name = "Landingpad Taxi Corner", desc = "", value = "-1872345847"))]
    LandingpadTaxiPieceCorner = -1872345847i32,
    #[strum(serialize = "ItemKitStairwell")]
    #[strum(props(name = "Kit (Stairwell)", desc = "", value = "-1868555784"))]
    ItemKitStairwell = -1868555784i32,
    #[strum(serialize = "ItemKitVendingMachineRefrigerated")]
    #[strum(
        props(
            name = "Kit (Vending Machine Refrigerated)",
            desc = "",
            value = "-1867508561"
        )
    )]
    ItemKitVendingMachineRefrigerated = -1867508561i32,
    #[strum(serialize = "ItemKitGasUmbilical")]
    #[strum(props(name = "Kit (Gas Umbilical)", desc = "", value = "-1867280568"))]
    ItemKitGasUmbilical = -1867280568i32,
    #[strum(serialize = "ItemBatteryCharger")]
    #[strum(
        props(
            name = "Kit (Battery Charger)",
            desc = "This kit produces a 5-slot <link=ThingItemBatteryCharger><color=green>Kit (Battery Charger)</color></link>.",
            value = "-1866880307"
        )
    )]
    ItemBatteryCharger = -1866880307i32,
    #[strum(serialize = "ItemMuffin")]
    #[strum(
        props(
            name = "Muffin",
            desc = "A delicious, semi-healthful snack, nothing comforts a <link=Stationeers><color=#0080FFFF>Stationeer</color></link> 800 million kilometers from home like a hand-made muffin.",
            value = "-1864982322"
        )
    )]
    ItemMuffin = -1864982322i32,
    #[strum(serialize = "ItemKitDynamicHydroponics")]
    #[strum(
        props(name = "Kit (Portable Hydroponics)", desc = "", value = "-1861154222")
    )]
    ItemKitDynamicHydroponics = -1861154222i32,
    #[strum(serialize = "StructureWallLight")]
    #[strum(props(name = "Wall Light", desc = "", value = "-1860064656"))]
    StructureWallLight = -1860064656i32,
    #[strum(serialize = "StructurePipeLiquidCorner")]
    #[strum(
        props(
            name = "Liquid Pipe (Corner)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidCorner><color=green>Insulated Liquid Pipe (Corner)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-1856720921"
        )
    )]
    StructurePipeLiquidCorner = -1856720921i32,
    #[strum(serialize = "ItemGasCanisterWater")]
    #[strum(props(name = "Liquid Canister (Water)", desc = "", value = "-1854861891"))]
    ItemGasCanisterWater = -1854861891i32,
    #[strum(serialize = "ItemKitLaunchMount")]
    #[strum(props(name = "Kit (Launch Mount)", desc = "", value = "-1854167549"))]
    ItemKitLaunchMount = -1854167549i32,
    #[strum(serialize = "DeviceLfoVolume")]
    #[strum(
        props(
            name = "Low frequency oscillator",
            desc = "The low frequency oscillator (or LFO) makes everything sound dark, twisted and crunchy by altering the shape of the waves output by a <link=ThingLogicStepSequencer8><color=green>Logic Step Sequencer</color></link>.\n      \nTo set up an LFO:\n\n1. Place the LFO unit\n2. Set the LFO output to a <link=ThingPassiveSpeaker><color=green>Passive Speaker</color></link>\n2. Set a sequencers' output to LFO - so the sequencer's signal runs through the LFO to a speaker.\n3. Place a <link=ThingStopWatch><color=green>Stop Watch</color></link> or use an existing one, then use a <link=ThingStructureLogicWriter><color=green>Logic Writer</color></link> to write it to the LFO.\n4. Use another logic writer to write the BPM to the LFO.\n5. You are ready. This is the future. You're in space. Make it sound cool.\n\nFor more info, check out the <link=MusicPage><color=#0080FFFF>music page</color></link>.",
            value = "-1844430312"
        )
    )]
    DeviceLfoVolume = -1844430312i32,
    #[strum(serialize = "StructureCableCornerH3")]
    #[strum(
        props(name = "Heavy Cable (3-Way Corner)", desc = "", value = "-1843379322")
    )]
    StructureCableCornerH3 = -1843379322i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerInner")]
    #[strum(
        props(
            name = "Composite Cladding (Angled Corner Inner)",
            desc = "",
            value = "-1841871763"
        )
    )]
    StructureCompositeCladdingAngledCornerInner = -1841871763i32,
    #[strum(serialize = "StructureHydroponicsTrayData")]
    #[strum(
        props(
            name = "Hydroponics Device",
            desc = "The <link=Agrizero><color=#0080FFFF>Agrizero</color></link> hydroponics device is the ideal vessel for growing a range of <link=OrganicPage><color=#0080FFFF>plantlife</color></link>. It must be supplied with <link=GasWater><color=#44AD83>Water</color></link> using a pipe network, and sufficient light to generate photosynthesis. \nIt can be automated using the <link=ThingStructureHarvie><color=green>Harvie</color></link>. Note that unlike the <link=ThingStructureHydroponicsTray><color=green>Hydroponics Tray</color></link>, these cannot be placed consecutively as they are considered devices rather than pure pipes. They do, however, allow data interrogation for logic systems.",
            value = "-1841632400"
        )
    )]
    StructureHydroponicsTrayData = -1841632400i32,
    #[strum(serialize = "ItemKitInsulatedPipeUtilityLiquid")]
    #[strum(
        props(
            name = "Kit (Insulated Pipe Utility Liquid)",
            desc = "",
            value = "-1831558953"
        )
    )]
    ItemKitInsulatedPipeUtilityLiquid = -1831558953i32,
    #[strum(serialize = "ItemKitWall")]
    #[strum(props(name = "Kit (Wall)", desc = "", value = "-1826855889"))]
    ItemKitWall = -1826855889i32,
    #[strum(serialize = "ItemWreckageAirConditioner1")]
    #[strum(props(name = "Wreckage Air Conditioner", desc = "", value = "-1826023284"))]
    ItemWreckageAirConditioner1 = -1826023284i32,
    #[strum(serialize = "ItemKitStirlingEngine")]
    #[strum(props(name = "Kit (Stirling Engine)", desc = "", value = "-1821571150"))]
    ItemKitStirlingEngine = -1821571150i32,
    #[strum(serialize = "StructureGasUmbilicalMale")]
    #[strum(
        props(
            name = "Umbilical (Gas)",
            desc = "0.Left\n1.Center\n2.Right",
            value = "-1814939203"
        )
    )]
    StructureGasUmbilicalMale = -1814939203i32,
    #[strum(serialize = "StructureSleeperRight")]
    #[strum(
        props(
            name = "Sleeper Right",
            desc = "A horizontal variant of the sleeper. Will keep players hydrated and fed while they are logged out - as long as a breathable atmosphere is provided.",
            value = "-1812330717"
        )
    )]
    StructureSleeperRight = -1812330717i32,
    #[strum(serialize = "StructureManualHatch")]
    #[strum(
        props(
            name = "Manual Hatch",
            desc = "Can be welded using a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> or <link=ThingItemArcWelder><color=green>Arc Welder</color></link> to lock it in the current state. Use the welder again to unlock.",
            value = "-1808154199"
        )
    )]
    StructureManualHatch = -1808154199i32,
    #[strum(serialize = "ItemOxite")]
    #[strum(
        props(
            name = "Ice (Oxite)",
            desc = "Oxite ice is largely composed of frozen <link=GasOxygen><color=#44AD83>Oxygen</color></link>, and found on many planets in the Solar System. Highly valuable and sought after, not all planets a <link=Stationeers><color=#0080FFFF>Stationeer</color></link> visits will have some. \n\nHighly sensitive to temperature, oxite will begin to melt as soon as it is mined, unless the temperature is below zero, or it is stored in the <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>, <link=ThingItemMiningBeltAdvanced><color=green>Mining Belt MK II</color></link> or devices like the <link=ThingStructureIceCrusher><color=green>Ice Crusher</color></link> or <link=ThingStructureFridgeSmall><color=green>Fridge Small</color></link>. When melting, oxite produces a mixture of <link=GasOxygen><color=#44AD83>Oxygen</color></link> and <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>.",
            value = "-1805394113"
        )
    )]
    ItemOxite = -1805394113i32,
    #[strum(serialize = "ItemKitLiquidTurboVolumePump")]
    #[strum(
        props(
            name = "Kit (Turbo Volume Pump - Liquid)",
            desc = "",
            value = "-1805020897"
        )
    )]
    ItemKitLiquidTurboVolumePump = -1805020897i32,
    #[strum(serialize = "StructureLiquidUmbilicalMale")]
    #[strum(
        props(
            name = "Umbilical (Liquid)",
            desc = "0.Left\n1.Center\n2.Right",
            value = "-1798420047"
        )
    )]
    StructureLiquidUmbilicalMale = -1798420047i32,
    #[strum(serialize = "StructurePipeMeter")]
    #[strum(
        props(
            name = "Pipe Meter",
            desc = "While the Stationeers program has, thus far, inspired little in the way of classical poetry, the following haiku was found etched, ironically, on a piece of pipe wreckage found on Vulcan:\n\"Humble pipe meter\nspeaks the truth, transmits pressure\nwithin any pipe\"",
            value = "-1798362329"
        )
    )]
    StructurePipeMeter = -1798362329i32,
    #[strum(serialize = "ItemKitUprightWindTurbine")]
    #[strum(
        props(name = "Kit (Upright Wind Turbine)", desc = "", value = "-1798044015")
    )]
    ItemKitUprightWindTurbine = -1798044015i32,
    #[strum(serialize = "ItemPipeRadiator")]
    #[strum(
        props(
            name = "Kit (Radiator)",
            desc = "This kit creates a <link=ThingStructurePipeRadiator><color=green>Pipe Convection Radiator</color></link>.",
            value = "-1796655088"
        )
    )]
    ItemPipeRadiator = -1796655088i32,
    #[strum(serialize = "StructureOverheadShortCornerLocker")]
    #[strum(props(name = "Overhead Corner Locker", desc = "", value = "-1794932560"))]
    StructureOverheadShortCornerLocker = -1794932560i32,
    #[strum(serialize = "ItemCableAnalyser")]
    #[strum(props(name = "Kit (Cable Analyzer)", desc = "", value = "-1792787349"))]
    ItemCableAnalyser = -1792787349i32,
    #[strum(serialize = "Landingpad_LiquidConnectorOutwardPiece")]
    #[strum(
        props(
            name = "Landingpad Liquid Output",
            desc = "Pumps liquid purchased from a trader out of the landing pad. You can increase the landing pad's liquid storage capacity by adding more <link=ThingLandingpad_GasCylinderTankPiece><color=green>Landingpad Gas Storage</color></link> to the landing pad.",
            value = "-1788929869"
        )
    )]
    LandingpadLiquidConnectorOutwardPiece = -1788929869i32,
    #[strum(serialize = "StructurePipeCorner")]
    #[strum(
        props(
            name = "Pipe (Corner)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCorner><color=green>Insulated Pipe (Corner)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-1785673561"
        )
    )]
    StructurePipeCorner = -1785673561i32,
    #[strum(serialize = "ItemKitSensor")]
    #[strum(props(name = "Kit (Sensors)", desc = "", value = "-1776897113"))]
    ItemKitSensor = -1776897113i32,
    #[strum(serialize = "ItemReusableFireExtinguisher")]
    #[strum(
        props(
            name = "Fire Extinguisher (Reusable)",
            desc = "Requires a canister filled with any inert liquid to opperate.",
            value = "-1773192190"
        )
    )]
    ItemReusableFireExtinguisher = -1773192190i32,
    #[strum(serialize = "CartridgeOreScanner")]
    #[strum(
        props(
            name = "Ore Scanner",
            desc = "When inserted into a <link=ThingItemTablet><color=green>Handheld Tablet</color></link> the scanner will display minerals hidden underground on the tablet.",
            value = "-1768732546"
        )
    )]
    CartridgeOreScanner = -1768732546i32,
    #[strum(serialize = "ItemPipeVolumePump")]
    #[strum(
        props(
            name = "Kit (Volume Pump)",
            desc = "This kit creates a <link=ThingStructureVolumePump><color=green>Volume Pump</color></link>.",
            value = "-1766301997"
        )
    )]
    ItemPipeVolumePump = -1766301997i32,
    #[strum(serialize = "StructureGrowLight")]
    #[strum(
        props(
            name = "Grow Light",
            desc = "<link=Agrizero><color=#0080FFFF>Agrizero's</color></link> leading hydroponic lighting system, the GrowUp UV light supplements sunshine in low light or sun-distant conditions. The unit adds growability over the space of a grid, so requires proximate placement to work. ",
            value = "-1758710260"
        )
    )]
    StructureGrowLight = -1758710260i32,
    #[strum(serialize = "ItemHardSuit")]
    #[strum(
        props(
            name = "Hardsuit",
            desc = "Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "-1758310454"
        )
    )]
    ItemHardSuit = -1758310454i32,
    #[strum(serialize = "StructureRailing")]
    #[strum(
        props(
            name = "Railing Industrial (Type 1)",
            desc = "\"Safety third.\"",
            value = "-1756913871"
        )
    )]
    StructureRailing = -1756913871i32,
    #[strum(serialize = "StructureCableJunction4Burnt")]
    #[strum(
        props(name = "Burnt Cable (4-Way Junction)", desc = "", value = "-1756896811")
    )]
    StructureCableJunction4Burnt = -1756896811i32,
    #[strum(serialize = "ItemCreditCard")]
    #[strum(props(name = "Credit Card", desc = "", value = "-1756772618"))]
    ItemCreditCard = -1756772618i32,
    #[strum(serialize = "ItemKitBlastDoor")]
    #[strum(props(name = "Kit (Blast Door)", desc = "", value = "-1755116240"))]
    ItemKitBlastDoor = -1755116240i32,
    #[strum(serialize = "ItemKitAutolathe")]
    #[strum(props(name = "Kit (Autolathe)", desc = "", value = "-1753893214"))]
    ItemKitAutolathe = -1753893214i32,
    #[strum(serialize = "ItemKitPassiveLargeRadiatorGas")]
    #[strum(props(name = "Kit (Medium Radiator)", desc = "", value = "-1752768283"))]
    ItemKitPassiveLargeRadiatorGas = -1752768283i32,
    #[strum(serialize = "StructurePictureFrameThinMountLandscapeSmall")]
    #[strum(
        props(
            name = "Picture Frame Thin Landscape Small",
            desc = "",
            value = "-1752493889"
        )
    )]
    StructurePictureFrameThinMountLandscapeSmall = -1752493889i32,
    #[strum(serialize = "ItemPipeHeater")]
    #[strum(
        props(
            name = "Pipe Heater Kit (Gas)",
            desc = "Creates a <link=ThingStructurePipeHeater><color=green>Pipe Heater (Gas)</color></link>.",
            value = "-1751627006"
        )
    )]
    ItemPipeHeater = -1751627006i32,
    #[strum(serialize = "ItemPureIceLiquidPollutant")]
    #[strum(
        props(
            name = "Pure Ice Liquid Pollutant",
            desc = "A frozen chunk of pure <link=GasLiquidPollutant><color=#44AD83>Liquid Pollutant</color></link>",
            value = "-1748926678"
        )
    )]
    ItemPureIceLiquidPollutant = -1748926678i32,
    #[strum(serialize = "ItemKitDrinkingFountain")]
    #[strum(props(name = "Kit (Drinking Fountain)", desc = "", value = "-1743663875"))]
    ItemKitDrinkingFountain = -1743663875i32,
    #[strum(serialize = "DynamicGasCanisterEmpty")]
    #[strum(
        props(
            name = "Portable Gas Tank",
            desc = "Portable gas tanks store gas. To refill one, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or bad things happen. Once it's full, you can refill a <link=ThingItemGasCanisterEmpty><color=green>Canister</color></link> by attaching it to the tank's striped section. Or you could vent the tank's variable flow rate valve into a room and create an atmosphere.",
            value = "-1741267161"
        )
    )]
    DynamicGasCanisterEmpty = -1741267161i32,
    #[strum(serialize = "ItemSpaceCleaner")]
    #[strum(
        props(
            name = "Space Cleaner",
            desc = "There was a time when humanity really wanted to keep space clean. That time has passed.",
            value = "-1737666461"
        )
    )]
    ItemSpaceCleaner = -1737666461i32,
    #[strum(serialize = "ItemAuthoringToolRocketNetwork")]
    #[strum(
        props(
            name = "<N:EN:ItemAuthoringToolRocketNetwork>",
            desc = "<N:EN:ItemAuthoringToolRocketNetwork>",
            value = "-1731627004"
        )
    )]
    ItemAuthoringToolRocketNetwork = -1731627004i32,
    #[strum(serialize = "ItemSensorProcessingUnitMesonScanner")]
    #[strum(
        props(
            name = "Sensor Processing Unit (T-Ray Scanner)",
            desc = "The T-Ray Scanner Sensor Processing Unit can be inserted into the <link=ThingItemSensorLenses><color=green>Sensor Lenses</color></link> to show an overlay of pipes and cables. This can be useful when building behind walls or other structures.",
            value = "-1730464583"
        )
    )]
    ItemSensorProcessingUnitMesonScanner = -1730464583i32,
    #[strum(serialize = "ItemWaterWallCooler")]
    #[strum(props(name = "Kit (Liquid Wall Cooler)", desc = "", value = "-1721846327"))]
    ItemWaterWallCooler = -1721846327i32,
    #[strum(serialize = "ItemPureIceLiquidCarbonDioxide")]
    #[strum(
        props(
            name = "Pure Ice Liquid Carbon Dioxide",
            desc = "A frozen chunk of pure <link=GasLiquidCarbonDioxide><color=#44AD83>Liquid Carbon Dioxide</color></link>",
            value = "-1715945725"
        )
    )]
    ItemPureIceLiquidCarbonDioxide = -1715945725i32,
    #[strum(serialize = "AccessCardRed")]
    #[strum(props(name = "Access Card (Red)", desc = "", value = "-1713748313"))]
    AccessCardRed = -1713748313i32,
    #[strum(serialize = "DynamicGasCanisterAir")]
    #[strum(
        props(
            name = "Portable Gas Tank (Air)",
            desc = "Portable gas tanks do one thing: store gas. But there's lots you can do with them. To refill the tank, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or bad things happen. Once it's full, you can refill a <link=ThingItemGasCanisterOxygen><color=green>Canister (Oxygen)</color></link> by attaching it to the tank's striped section. Or you could vent the tank's variable flow rate valve into a room and create an atmosphere. They also attach to rovers and rockets. Alternatively, kick it over and practice barrel rolling. The possibilities are endless.",
            value = "-1713611165"
        )
    )]
    DynamicGasCanisterAir = -1713611165i32,
    #[strum(serialize = "StructureMotionSensor")]
    #[strum(
        props(
            name = "Motion Sensor",
            desc = "Originally developed to monitor dance marathons, the motion sensor can also be connected to <link=LogicPage><color=#0080FFFF>Logic</color></link> systems for security purposes, automatic lighting, doors and various other applications.\nThe sensor activates whenever a player enters the grid it is placed on.",
            value = "-1713470563"
        )
    )]
    StructureMotionSensor = -1713470563i32,
    #[strum(serialize = "ItemCookedPowderedEggs")]
    #[strum(
        props(
            name = "Powdered Eggs",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "-1712264413"
        )
    )]
    ItemCookedPowderedEggs = -1712264413i32,
    #[strum(serialize = "ItemGasCanisterNitrousOxide")]
    #[strum(props(name = "Gas Canister (Sleeping)", desc = "", value = "-1712153401"))]
    ItemGasCanisterNitrousOxide = -1712153401i32,
    #[strum(serialize = "ItemKitHeatExchanger")]
    #[strum(props(name = "Kit Heat Exchanger", desc = "", value = "-1710540039"))]
    ItemKitHeatExchanger = -1710540039i32,
    #[strum(serialize = "ItemPureIceNitrogen")]
    #[strum(
        props(
            name = "Pure Ice Nitrogen",
            desc = "A frozen chunk of pure <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>",
            value = "-1708395413"
        )
    )]
    ItemPureIceNitrogen = -1708395413i32,
    #[strum(serialize = "ItemKitPipeRadiatorLiquid")]
    #[strum(
        props(name = "Kit (Pipe Radiator Liquid)", desc = "", value = "-1697302609")
    )]
    ItemKitPipeRadiatorLiquid = -1697302609i32,
    #[strum(serialize = "StructureInLineTankGas1x1")]
    #[strum(
        props(
            name = "In-Line Tank Small Gas",
            desc = "A small expansion tank that increases the volume of a pipe network.",
            value = "-1693382705"
        )
    )]
    StructureInLineTankGas1X1 = -1693382705i32,
    #[strum(serialize = "SeedBag_Rice")]
    #[strum(
        props(
            name = "Rice Seeds",
            desc = "Grow some <link=ThingItemRice><color=green>Rice</color></link>.",
            value = "-1691151239"
        )
    )]
    SeedBagRice = -1691151239i32,
    #[strum(serialize = "StructurePictureFrameThickPortraitLarge")]
    #[strum(
        props(
            name = "Picture Frame Thick Portrait Large",
            desc = "",
            value = "-1686949570"
        )
    )]
    StructurePictureFrameThickPortraitLarge = -1686949570i32,
    #[strum(serialize = "ApplianceDeskLampLeft")]
    #[strum(props(name = "Appliance Desk Lamp Left", desc = "", value = "-1683849799"))]
    ApplianceDeskLampLeft = -1683849799i32,
    #[strum(serialize = "ItemWreckageWallCooler1")]
    #[strum(props(name = "Wreckage Wall Cooler", desc = "", value = "-1682930158"))]
    ItemWreckageWallCooler1 = -1682930158i32,
    #[strum(serialize = "StructureGasUmbilicalFemale")]
    #[strum(props(name = "Umbilical Socket (Gas)", desc = "", value = "-1680477930"))]
    StructureGasUmbilicalFemale = -1680477930i32,
    #[strum(serialize = "ItemGasFilterWaterInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Water)",
            desc = "A filter that selectively targets Water. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-1678456554"
        )
    )]
    ItemGasFilterWaterInfinite = -1678456554i32,
    #[strum(serialize = "StructurePassthroughHeatExchangerGasToGas")]
    #[strum(
        props(
            name = "CounterFlow Heat Exchanger - Gas + Gas",
            desc = "Exchange heat from one pipe network to another. By drawing down the pressure of the outputs with a pump or regulator and regulating input pressures, the temperatures of two counterflowing networks can be effectively exchanged.\n      Balancing the throughput of both inputs is key to creating a good exhange of temperatures.",
            value = "-1674187440"
        )
    )]
    StructurePassthroughHeatExchangerGasToGas = -1674187440i32,
    #[strum(serialize = "StructureAutomatedOven")]
    #[strum(props(name = "Automated Oven", desc = "", value = "-1672404896"))]
    StructureAutomatedOven = -1672404896i32,
    #[strum(serialize = "StructureElectrolyzer")]
    #[strum(
        props(
            name = "Electrolyzer",
            desc = "The <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> Electrolyzer splits <link=GasWater><color=#44AD83>Water</color></link> into hydrogen and <link=GasOxygen><color=#44AD83>Oxygen</color></link>. Employing unknown proprietary technology, the device uses water's latent heat as the energy to drive the electrosis process. If there is a downside to this near-miraculous fission, it's that the device is limited by the quantity of power available, which is used to maintain the temperature output. In other words, the machine works best with hot gas.",
            value = "-1668992663"
        )
    )]
    StructureElectrolyzer = -1668992663i32,
    #[strum(serialize = "MonsterEgg")]
    #[strum(
        props(
            name = "<N:EN:MonsterEgg>",
            desc = "<N:EN:MonsterEgg>",
            value = "-1667675295"
        )
    )]
    MonsterEgg = -1667675295i32,
    #[strum(serialize = "ItemMiningDrillHeavy")]
    #[strum(
        props(
            name = "Mining Drill (Heavy)",
            desc = "Sometimes mining trips require something a little bigger to bring home the goods. This scaled up version of the <link=Recurso><color=#0080FFFF>Recurso</color></link> 'Topo' design <link=ThingItemMiningDrill><color=green>Mining Drill</color></link> can literally move mountains. The heavy mining drill will remove more ground and mine <link=OrePage><color=#0080FFFF>ore</color></link> more quickly than the standard mining drill. The heavy mining drill is also resilient to temperature and pressure. So no matter what planet or extreme weather conditions may be present, the Recurso heavy mining drill will get the job done.",
            value = "-1663349918"
        )
    )]
    ItemMiningDrillHeavy = -1663349918i32,
    #[strum(serialize = "ItemAstroloySheets")]
    #[strum(props(name = "Astroloy Sheets", desc = "", value = "-1662476145"))]
    ItemAstroloySheets = -1662476145i32,
    #[strum(serialize = "ItemWreckageTurbineGenerator1")]
    #[strum(
        props(name = "Wreckage Turbine Generator", desc = "", value = "-1662394403")
    )]
    ItemWreckageTurbineGenerator1 = -1662394403i32,
    #[strum(serialize = "ItemMiningBackPack")]
    #[strum(props(name = "Mining Backpack", desc = "", value = "-1650383245"))]
    ItemMiningBackPack = -1650383245i32,
    #[strum(serialize = "ItemSprayCanGrey")]
    #[strum(
        props(
            name = "Spray Paint (Grey)",
            desc = "Arguably the most popular color in the universe, grey was invented so designers had something to do.",
            value = "-1645266981"
        )
    )]
    ItemSprayCanGrey = -1645266981i32,
    #[strum(serialize = "ItemReagentMix")]
    #[strum(
        props(
            name = "Reagent Mix",
            desc = "Reagent mix is pure potential. A slurry of undifferentiated <link=OrePage><color=#0080FFFF>ores</color></link>, it is output by the <link=ThingStructureRecycler><color=green>Recycler</color></link> and can be fed into the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link> to separate and recover the individual materials. Reagent mix is also output by the <link=ThingStructureFurnace><color=green>Furnace</color></link> when the current contents are ejected without smelting a specific ingot.",
            value = "-1641500434"
        )
    )]
    ItemReagentMix = -1641500434i32,
    #[strum(serialize = "CartridgeAccessController")]
    #[strum(
        props(name = "Cartridge (Access Controller)", desc = "", value = "-1634532552")
    )]
    CartridgeAccessController = -1634532552i32,
    #[strum(serialize = "StructureRecycler")]
    #[strum(
        props(
            name = "Recycler",
            desc = "A device for collecting the raw resources while destroying an item. Produces <link=ThingItemReagentMix><color=green>Reagent Mix</color></link> containing packages of reagents. Pass these through the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link> to gain back the source ores. Plants and organic matter passed through will create Biomass, which when passed through the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link> will produce <link=ThingItemBiomass><color=green>Biomass</color></link>.",
            value = "-1633947337"
        )
    )]
    StructureRecycler = -1633947337i32,
    #[strum(serialize = "StructureSmallTableBacklessDouble")]
    #[strum(
        props(name = "Small (Table Backless Double)", desc = "", value = "-1633000411")
    )]
    StructureSmallTableBacklessDouble = -1633000411i32,
    #[strum(serialize = "ItemKitRocketGasFuelTank")]
    #[strum(
        props(name = "Kit (Rocket Gas Fuel Tank)", desc = "", value = "-1629347579")
    )]
    ItemKitRocketGasFuelTank = -1629347579i32,
    #[strum(serialize = "StructureStairwellFrontPassthrough")]
    #[strum(
        props(name = "Stairwell (Front Passthrough)", desc = "", value = "-1625452928")
    )]
    StructureStairwellFrontPassthrough = -1625452928i32,
    #[strum(serialize = "StructureCableJunctionBurnt")]
    #[strum(props(name = "Burnt Cable (Junction)", desc = "", value = "-1620686196"))]
    StructureCableJunctionBurnt = -1620686196i32,
    #[strum(serialize = "ItemKitPipe")]
    #[strum(props(name = "Kit (Pipe)", desc = "", value = "-1619793705"))]
    ItemKitPipe = -1619793705i32,
    #[strum(serialize = "ItemPureIce")]
    #[strum(
        props(
            name = "Pure Ice Water",
            desc = "A frozen chunk of pure <link=GasWater><color=#44AD83>Water</color></link>",
            value = "-1616308158"
        )
    )]
    ItemPureIce = -1616308158i32,
    #[strum(serialize = "StructureBasketHoop")]
    #[strum(props(name = "Basket Hoop", desc = "", value = "-1613497288"))]
    StructureBasketHoop = -1613497288i32,
    #[strum(serialize = "StructureWallPaddedThinNoBorder")]
    #[strum(
        props(name = "Wall (Padded Thin No Border)", desc = "", value = "-1611559100")
    )]
    StructureWallPaddedThinNoBorder = -1611559100i32,
    #[strum(serialize = "StructureTankBig")]
    #[strum(props(name = "Large Tank", desc = "", value = "-1606848156"))]
    StructureTankBig = -1606848156i32,
    #[strum(serialize = "StructureInsulatedTankConnectorLiquid")]
    #[strum(
        props(name = "Insulated Tank Connector Liquid", desc = "", value = "-1602030414")
    )]
    StructureInsulatedTankConnectorLiquid = -1602030414i32,
    #[strum(serialize = "ItemKitTurbineGenerator")]
    #[strum(props(name = "Kit (Turbine Generator)", desc = "", value = "-1590715731"))]
    ItemKitTurbineGenerator = -1590715731i32,
    #[strum(serialize = "ItemKitCrateMkII")]
    #[strum(props(name = "Kit (Crate Mk II)", desc = "", value = "-1585956426"))]
    ItemKitCrateMkIi = -1585956426i32,
    #[strum(serialize = "StructureRefrigeratedVendingMachine")]
    #[strum(
        props(
            name = "Refrigerated Vending Machine",
            desc = "The refrigerated OmniKool vending machine is an advanced version of the standard <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link>, which maintains an optimum pressure and constant temperature of -130 degrees C, to prevent food spoilage. It can hold up to 100 stacks.\nThe OmniKool also has an in-built <link=ThingStructureStacker><color=green>Stacker</color></link>, allowing players to set the stack sizes of any items ADDED to the device. The unit's default stack size is 50.\nNOTE: altering stack sizes DOES NOT update existing stacks within the machine, only those subsequently added. ",
            value = "-1577831321"
        )
    )]
    StructureRefrigeratedVendingMachine = -1577831321i32,
    #[strum(serialize = "ItemFlowerBlue")]
    #[strum(props(name = "Flower (Blue)", desc = "", value = "-1573623434"))]
    ItemFlowerBlue = -1573623434i32,
    #[strum(serialize = "ItemWallCooler")]
    #[strum(
        props(
            name = "Kit (Wall Cooler)",
            desc = "This kit creates a <link=ThingStructureWallCooler><color=green>Wall Cooler</color></link>.",
            value = "-1567752627"
        )
    )]
    ItemWallCooler = -1567752627i32,
    #[strum(serialize = "StructureSolarPanel45")]
    #[strum(
        props(
            name = "Solar Panel (Angled)",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai</color></link> basic solar panels generate power from sunlight, sitting at 45 degrees to the ground. Their efficiency is reduced during storms and when damaged. You can repair these using some trusty <link=ThingItemDuctTape><color=green>Duct Tape</color></link>.",
            value = "-1554349863"
        )
    )]
    StructureSolarPanel45 = -1554349863i32,
    #[strum(serialize = "ItemGasCanisterPollutants")]
    #[strum(props(name = "Canister (Pollutants)", desc = "", value = "-1552586384"))]
    ItemGasCanisterPollutants = -1552586384i32,
    #[strum(serialize = "CartridgeAtmosAnalyser")]
    #[strum(
        props(
            name = "Atmos Analyzer",
            desc = "The Lorenz atmos analyzer is a multi-functional mass-spectrometer designed by <link=ExMin><color=#0080FFFF>ExMin</color></link> for use with the OreCore <link=ThingItemTablet><color=green>Handheld Tablet</color></link>. It displays the pressure, concentration and molar quantity of <link=GasPage><color=#0080FFFF>gas</color></link> in rooms, tanks, or pipe networks.",
            value = "-1550278665"
        )
    )]
    CartridgeAtmosAnalyser = -1550278665i32,
    #[strum(serialize = "StructureWallPaddedArchLightsFittings")]
    #[strum(
        props(
            name = "Wall (Padded Arch Lights Fittings)",
            desc = "",
            value = "-1546743960"
        )
    )]
    StructureWallPaddedArchLightsFittings = -1546743960i32,
    #[strum(serialize = "StructureSolarPanelDualReinforced")]
    #[strum(
        props(
            name = "Solar Panel (Heavy Dual)",
            desc = "This solar panel is resistant to storm damage.",
            value = "-1545574413"
        )
    )]
    StructureSolarPanelDualReinforced = -1545574413i32,
    #[strum(serialize = "StructureCableCorner4")]
    #[strum(props(name = "Cable (4-Way Corner)", desc = "", value = "-1542172466"))]
    StructureCableCorner4 = -1542172466i32,
    #[strum(serialize = "StructurePressurePlateSmall")]
    #[strum(props(name = "Trigger Plate (Small)", desc = "", value = "-1536471028"))]
    StructurePressurePlateSmall = -1536471028i32,
    #[strum(serialize = "StructureFlashingLight")]
    #[strum(
        props(
            name = "Flashing Light",
            desc = "Few objects or ideas are as clearly and transparently named as the Flashing Light, although fans of scrupulous accuracy have been known to refer to it by its full, official title: 'Default Yellow Flashing Light'.",
            value = "-1535893860"
        )
    )]
    StructureFlashingLight = -1535893860i32,
    #[strum(serialize = "StructureFuselageTypeA2")]
    #[strum(props(name = "Fuselage (Type A2)", desc = "", value = "-1533287054"))]
    StructureFuselageTypeA2 = -1533287054i32,
    #[strum(serialize = "ItemPipeDigitalValve")]
    #[strum(
        props(
            name = "Kit (Digital Valve)",
            desc = "This kit creates a <link=ThingStructureDigitalValve><color=green>Digital Valve</color></link>.",
            value = "-1532448832"
        )
    )]
    ItemPipeDigitalValve = -1532448832i32,
    #[strum(serialize = "StructureCableJunctionH5")]
    #[strum(
        props(name = "Heavy Cable (5-Way Junction)", desc = "", value = "-1530571426")
    )]
    StructureCableJunctionH5 = -1530571426i32,
    #[strum(serialize = "StructureFlagSmall")]
    #[strum(props(name = "Small Flag", desc = "", value = "-1529819532"))]
    StructureFlagSmall = -1529819532i32,
    #[strum(serialize = "StopWatch")]
    #[strum(props(name = "Stop Watch", desc = "", value = "-1527229051"))]
    StopWatch = -1527229051i32,
    #[strum(serialize = "ItemUraniumOre")]
    #[strum(
        props(
            name = "Ore (Uranium)",
            desc = "In 1934, Enrico Fermi noticed that bombarding uranium with neutrons produced a burst of beta rays, and a new material. This process was named 'nuclear fission', and resulted in cheap energy, the Cold War, and countless thousand deaths. While reasonably common throughout the Solar System, <link=Stationeers><color=#0080FFFF>Stationeers</color></link> are wary of the material.",
            value = "-1516581844"
        )
    )]
    ItemUraniumOre = -1516581844i32,
    #[strum(serialize = "Landingpad_ThreshholdPiece")]
    #[strum(props(name = "Landingpad Threshhold", desc = "", value = "-1514298582"))]
    LandingpadThreshholdPiece = -1514298582i32,
    #[strum(serialize = "ItemFlowerGreen")]
    #[strum(props(name = "Flower (Green)", desc = "", value = "-1513337058"))]
    ItemFlowerGreen = -1513337058i32,
    #[strum(serialize = "StructureCompositeCladdingAngled")]
    #[strum(
        props(name = "Composite Cladding (Angled)", desc = "", value = "-1513030150")
    )]
    StructureCompositeCladdingAngled = -1513030150i32,
    #[strum(serialize = "StructureChairThickSingle")]
    #[strum(props(name = "Chair (Thick Single)", desc = "", value = "-1510009608"))]
    StructureChairThickSingle = -1510009608i32,
    #[strum(serialize = "StructureInsulatedPipeCrossJunction5")]
    #[strum(
        props(
            name = "Insulated Pipe (5-Way Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "-1505147578"
        )
    )]
    StructureInsulatedPipeCrossJunction5 = -1505147578i32,
    #[strum(serialize = "ItemNitrice")]
    #[strum(
        props(
            name = "Ice (Nitrice)",
            desc = "Nitrice is the nickname given to solid <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> Ice, and found on many planets and moons in the Solar System. Given the inert nature of the <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> it produces, the ice is useful when making breathable atmospheres with low flammability.\n\nHighly sensitive to temperature, nitrice will begin to melt as soon as it is mined, unless the temperature is below zero, or it is stored in the <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>, <link=ThingItemMiningBeltAdvanced><color=green>Mining Belt MK II</color></link> or devices like the <link=ThingStructureIceCrusher><color=green>Ice Crusher</color></link> or <link=ThingStructureFridgeSmall><color=green>Fridge Small</color></link>.",
            value = "-1499471529"
        )
    )]
    ItemNitrice = -1499471529i32,
    #[strum(serialize = "StructureCargoStorageSmall")]
    #[strum(props(name = "Cargo Storage (Small)", desc = "", value = "-1493672123"))]
    StructureCargoStorageSmall = -1493672123i32,
    #[strum(serialize = "StructureLogicCompare")]
    #[strum(
        props(
            name = "Logic Compare",
            desc = "0.Equals\n1.Greater\n2.Less\n3.NotEquals",
            value = "-1489728908"
        )
    )]
    StructureLogicCompare = -1489728908i32,
    #[strum(serialize = "Landingpad_TaxiPieceStraight")]
    #[strum(props(name = "Landingpad Taxi Straight", desc = "", value = "-1477941080"))]
    LandingpadTaxiPieceStraight = -1477941080i32,
    #[strum(serialize = "StructurePassthroughHeatExchangerLiquidToLiquid")]
    #[strum(
        props(
            name = "CounterFlow Heat Exchanger - Liquid + Liquid",
            desc = "Exchange heat from one pipe network to another. By drawing down the pressure of the outputs with a pump or regulator and regulating input pressures, the temperatures of two counterflowing networks can be effectively exchanged.\n        Balancing the throughput of both inputs is key to creating a good exchange of temperatures.",
            value = "-1472829583"
        )
    )]
    StructurePassthroughHeatExchangerLiquidToLiquid = -1472829583i32,
    #[strum(serialize = "ItemKitCompositeCladding")]
    #[strum(props(name = "Kit (Cladding)", desc = "", value = "-1470820996"))]
    ItemKitCompositeCladding = -1470820996i32,
    #[strum(serialize = "StructureChuteInlet")]
    #[strum(
        props(
            name = "Chute Inlet",
            desc = "The aim for any <link=Stationeers><color=#0080FFFF>Stationeer</color></link> is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nThe chute inlet is an aperture by which items can be introduced to <link=ImportExportPage><color=#0080FFFF>import/export</color></link> networks. Note that its origins in zero-gravity mining means chute inlets are unpowered and permanently open, rather than interactable, allowing objects to be thrown in. They can be connected to <link=LogicUnitPage><color=#0080FFFF>logic</color></link> systems to monitor throughput.",
            value = "-1469588766"
        )
    )]
    StructureChuteInlet = -1469588766i32,
    #[strum(serialize = "StructureSleeper")]
    #[strum(props(name = "Sleeper", desc = "", value = "-1467449329"))]
    StructureSleeper = -1467449329i32,
    #[strum(serialize = "CartridgeElectronicReader")]
    #[strum(props(name = "eReader", desc = "", value = "-1462180176"))]
    CartridgeElectronicReader = -1462180176i32,
    #[strum(serialize = "StructurePictureFrameThickMountPortraitLarge")]
    #[strum(
        props(
            name = "Picture Frame Thick Mount Portrait Large",
            desc = "",
            value = "-1459641358"
        )
    )]
    StructurePictureFrameThickMountPortraitLarge = -1459641358i32,
    #[strum(serialize = "ItemSteelFrames")]
    #[strum(
        props(
            name = "Steel Frames",
            desc = "An advanced and stronger version of <link=ThingItemIronFrames><color=green>Iron Frames</color></link>, steel frames are placed by right-clicking. To complete construction, use <link=ThingItemSteelSheets><color=green>Steel Sheets</color></link> and a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> in your active hand.",
            value = "-1448105779"
        )
    )]
    ItemSteelFrames = -1448105779i32,
    #[strum(serialize = "StructureChuteFlipFlopSplitter")]
    #[strum(
        props(
            name = "Chute Flip Flop Splitter",
            desc = "A chute that toggles between two outputs",
            value = "-1446854725"
        )
    )]
    StructureChuteFlipFlopSplitter = -1446854725i32,
    #[strum(serialize = "StructurePictureFrameThickLandscapeLarge")]
    #[strum(
        props(
            name = "Picture Frame Thick Landscape Large",
            desc = "",
            value = "-1434523206"
        )
    )]
    StructurePictureFrameThickLandscapeLarge = -1434523206i32,
    #[strum(serialize = "ItemKitAdvancedComposter")]
    #[strum(props(name = "Kit (Advanced Composter)", desc = "", value = "-1431998347"))]
    ItemKitAdvancedComposter = -1431998347i32,
    #[strum(serialize = "StructureLiquidTankBigInsulated")]
    #[strum(props(name = "Insulated Liquid Tank Big", desc = "", value = "-1430440215"))]
    StructureLiquidTankBigInsulated = -1430440215i32,
    #[strum(serialize = "StructureEvaporationChamber")]
    #[strum(
        props(
            name = "Evaporation Chamber",
            desc = "A device for safely evaporating liquids into gasses. Liquids and Gasses will both exist safely inside the device. Lowering the pressure target of the in-built back pressure regulator using the setting wheel will change the boiling temperature of liquids inside.\n        The secondary gas input on the left is a heat-exchanger input and allows for heat exchange between the secondary input pipe and the internal atmosphere of the Evaporation Chamber. \n        Paired with <link=ThingStructureCondensationChamber><color=green>Condensation Chamber</color></link> Stationeers can exploit the phase change properties of gases to build a DIY air conditioner.",
            value = "-1429782576"
        )
    )]
    StructureEvaporationChamber = -1429782576i32,
    #[strum(serialize = "StructureWallGeometryTMirrored")]
    #[strum(
        props(name = "Wall (Geometry T Mirrored)", desc = "", value = "-1427845483")
    )]
    StructureWallGeometryTMirrored = -1427845483i32,
    #[strum(serialize = "KitchenTableShort")]
    #[strum(props(name = "Kitchen Table (Short)", desc = "", value = "-1427415566"))]
    KitchenTableShort = -1427415566i32,
    #[strum(serialize = "StructureChairRectangleSingle")]
    #[strum(props(name = "Chair (Rectangle Single)", desc = "", value = "-1425428917"))]
    StructureChairRectangleSingle = -1425428917i32,
    #[strum(serialize = "StructureTransformer")]
    #[strum(
        props(
            name = "Transformer (Large)",
            desc = "The large <link=Norsec><color=#0080FFFF>Norsec</color></link> transformer is a critical component of extended <link=ElectronicPage><color=#0080FFFF>electrical</color></link> networks, controlling the maximum power that will flow down a cable. To prevent overloading, output can be set from 0 to 50,000W. \nNote that transformers operate as data isolators, preventing data flowing into any network beyond it.",
            value = "-1423212473"
        )
    )]
    StructureTransformer = -1423212473i32,
    #[strum(serialize = "StructurePictureFrameThinLandscapeLarge")]
    #[strum(
        props(
            name = "Picture Frame Thin Landscape Large",
            desc = "",
            value = "-1418288625"
        )
    )]
    StructurePictureFrameThinLandscapeLarge = -1418288625i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerInnerLong")]
    #[strum(
        props(
            name = "Composite Cladding (Angled Corner Inner Long)",
            desc = "",
            value = "-1417912632"
        )
    )]
    StructureCompositeCladdingAngledCornerInnerLong = -1417912632i32,
    #[strum(serialize = "ItemPlantEndothermic_Genepool2")]
    #[strum(
        props(
            name = "Winterspawn (Beta variant)",
            desc = "<link=Agrizero><color=#0080FFFF>Agrizero's</color></link> Winterspawn atmospheric bio-processor is a recent addition to their catalog of genespliced environmental decorations. Using ambient heat to split <link=GasWater><color=#44AD83>Water</color></link> into <link=GasVolatiles><color=#44AD83>Volatiles</color></link> and <link=GasOxygen><color=#44AD83>Oxygen</color></link>, the Winterspawn cools its surroundings when supplied with sufficient <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>. The beta variant has a peak cooling and electrolysis capacity of 150Watts and is most efficient operating in air temperatures of 14 to 24 Degrees Celsius.",
            value = "-1414203269"
        )
    )]
    ItemPlantEndothermicGenepool2 = -1414203269i32,
    #[strum(serialize = "ItemFlowerOrange")]
    #[strum(props(name = "Flower (Orange)", desc = "", value = "-1411986716"))]
    ItemFlowerOrange = -1411986716i32,
    #[strum(serialize = "AccessCardBlue")]
    #[strum(props(name = "Access Card (Blue)", desc = "", value = "-1411327657"))]
    AccessCardBlue = -1411327657i32,
    #[strum(serialize = "StructureWallSmallPanelsOpen")]
    #[strum(props(name = "Wall (Small Panels Open)", desc = "", value = "-1407480603"))]
    StructureWallSmallPanelsOpen = -1407480603i32,
    #[strum(serialize = "ItemNickelIngot")]
    #[strum(props(name = "Ingot (Nickel)", desc = "", value = "-1406385572"))]
    ItemNickelIngot = -1406385572i32,
    #[strum(serialize = "StructurePipeCrossJunction")]
    #[strum(
        props(
            name = "Pipe (Cross Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCrossJunction><color=green>Insulated Pipe (Cross Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-1405295588"
        )
    )]
    StructurePipeCrossJunction = -1405295588i32,
    #[strum(serialize = "StructureCableJunction6")]
    #[strum(
        props(
            name = "Cable (6-Way Junction)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> duty - so much so, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link> during the 3rd Decannual Stationeer Solar Conference.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "-1404690610"
        )
    )]
    StructureCableJunction6 = -1404690610i32,
    #[strum(serialize = "ItemPassiveVentInsulated")]
    #[strum(
        props(name = "Kit (Insulated Passive Vent)", desc = "", value = "-1397583760")
    )]
    ItemPassiveVentInsulated = -1397583760i32,
    #[strum(serialize = "ItemKitChairs")]
    #[strum(props(name = "Kit (Chairs)", desc = "", value = "-1394008073"))]
    ItemKitChairs = -1394008073i32,
    #[strum(serialize = "StructureBatteryLarge")]
    #[strum(
        props(
            name = "Station Battery (Large)",
            desc = "Providing even better large-scale, reliable power storage than the {THING;StructureBattery}, the <link=Sinotai><color=#0080FFFF>Sinotai</color></link> 'Da Dianchi' large station battery is the heart of most <link=Stationeers><color=#0080FFFF>Stationeer</color></link> bases. \nThere are a variety of cautions to the design of electrical systems using batteries, and every experienced Stationeer has a story to tell, hence the Stationeer adage: 'Dianzi cooks, but it also frys.' \n<size=120%><b>POWER OUTPUT</b></size>\nAble to store up to 9000001 watts of power, there are no practical limits to its throughput, hence it is wise to use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>. Seasoned electrical engineers will also laugh in the face of those who fail to separate out their power generation networks using an <link=ThingStructureAreaPowerControl><color=green>Area Power Control</color></link> and <link=ThingStructureTransformer><color=green>Transformer (Large)</color></link>. ",
            value = "-1388288459"
        )
    )]
    StructureBatteryLarge = -1388288459i32,
    #[strum(serialize = "ItemGasFilterNitrogenL")]
    #[strum(props(name = "Heavy Filter (Nitrogen)", desc = "", value = "-1387439451"))]
    ItemGasFilterNitrogenL = -1387439451i32,
    #[strum(serialize = "KitchenTableTall")]
    #[strum(props(name = "Kitchen Table (Tall)", desc = "", value = "-1386237782"))]
    KitchenTableTall = -1386237782i32,
    #[strum(serialize = "StructureCapsuleTankGas")]
    #[strum(props(name = "Gas Capsule Tank Small", desc = "", value = "-1385712131"))]
    StructureCapsuleTankGas = -1385712131i32,
    #[strum(serialize = "StructureCryoTubeVertical")]
    #[strum(
        props(
            name = "Cryo Tube Vertical",
            desc = "The vertical variant of the cryo tube. Will heal players and organs as well as revive dead players when provided with an atmosphere of Nitrogen below -150C.",
            value = "-1381321828"
        )
    )]
    StructureCryoTubeVertical = -1381321828i32,
    #[strum(serialize = "StructureWaterWallCooler")]
    #[strum(props(name = "Liquid Wall Cooler", desc = "", value = "-1369060582"))]
    StructureWaterWallCooler = -1369060582i32,
    #[strum(serialize = "ItemKitTables")]
    #[strum(props(name = "Kit (Tables)", desc = "", value = "-1361598922"))]
    ItemKitTables = -1361598922i32,
    #[strum(serialize = "StructureLargeHangerDoor")]
    #[strum(
        props(
            name = "Large Hangar Door",
            desc = "1 x 3 modular door piece for building hangar doors.",
            value = "-1351081801"
        )
    )]
    StructureLargeHangerDoor = -1351081801i32,
    #[strum(serialize = "ItemGoldOre")]
    #[strum(
        props(
            name = "Ore (Gold)",
            desc = "Surprisingly common throughout the Solar System, <link=ReagentGold><color=#B566FF>Gold</color></link> is thought to originate in the heart of supernovas, gathering as dust in the early stages of solar formation, then incorporating into the slowly accreting planetary bodies. Now a prized element in <link=Stationeers><color=#0080FFFF>Stationeer</color></link> construction, <link=ReagentGold><color=#B566FF>Gold</color></link> is valued not for its beauty, but its reliability: inert, durable, conductive and highly stable, gold's strength is that it does nothing.",
            value = "-1348105509"
        )
    )]
    ItemGoldOre = -1348105509i32,
    #[strum(serialize = "ItemCannedMushroom")]
    #[strum(
        props(
            name = "Canned Mushroom",
            desc = "Made in an <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link> or <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link>, using <link=ThingItemCookedMushroom><color=green>Cooked Mushroom</color></link>  and a <link=ThingItemEmptyCan><color=green>Empty Can</color></link>, delicious mushroom soup is fairly high in nutrition, and does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "-1344601965"
        )
    )]
    ItemCannedMushroom = -1344601965i32,
    #[strum(serialize = "AppliancePaintMixer")]
    #[strum(props(name = "Paint Mixer", desc = "", value = "-1339716113"))]
    AppliancePaintMixer = -1339716113i32,
    #[strum(serialize = "AccessCardGray")]
    #[strum(props(name = "Access Card (Gray)", desc = "", value = "-1339479035"))]
    AccessCardGray = -1339479035i32,
    #[strum(serialize = "StructureChuteDigitalValveRight")]
    #[strum(
        props(
            name = "Chute Digital Valve Right",
            desc = "The Digital Chute Valve will stop the flow of materials when set to closed and when set to open, will act like a straight chute. The valve will automatically close after a certain number of items have passed through. This threshold can be set using the dial.",
            value = "-1337091041"
        )
    )]
    StructureChuteDigitalValveRight = -1337091041i32,
    #[strum(serialize = "ItemSugarCane")]
    #[strum(props(name = "Sugarcane", desc = "", value = "-1335056202"))]
    ItemSugarCane = -1335056202i32,
    #[strum(serialize = "ItemKitSmallDirectHeatExchanger")]
    #[strum(
        props(
            name = "Kit (Small Direct Heat Exchanger)",
            desc = "",
            value = "-1332682164"
        )
    )]
    ItemKitSmallDirectHeatExchanger = -1332682164i32,
    #[strum(serialize = "AccessCardBlack")]
    #[strum(props(name = "Access Card (Black)", desc = "", value = "-1330388999"))]
    AccessCardBlack = -1330388999i32,
    #[strum(serialize = "StructureLogicWriter")]
    #[strum(props(name = "Logic Writer", desc = "", value = "-1326019434"))]
    StructureLogicWriter = -1326019434i32,
    #[strum(serialize = "StructureLogicWriterSwitch")]
    #[strum(props(name = "Logic Writer Switch", desc = "", value = "-1321250424"))]
    StructureLogicWriterSwitch = -1321250424i32,
    #[strum(serialize = "StructureWallIron04")]
    #[strum(props(name = "Iron Wall (Type 4)", desc = "", value = "-1309433134"))]
    StructureWallIron04 = -1309433134i32,
    #[strum(serialize = "ItemPureIceLiquidVolatiles")]
    #[strum(
        props(
            name = "Pure Ice Liquid Volatiles",
            desc = "A frozen chunk of pure <link=GasLiquidVolatiles><color=#44AD83>Liquid Volatiles</color></link>",
            value = "-1306628937"
        )
    )]
    ItemPureIceLiquidVolatiles = -1306628937i32,
    #[strum(serialize = "StructureWallLightBattery")]
    #[strum(props(name = "Wall Light (Battery)", desc = "", value = "-1306415132"))]
    StructureWallLightBattery = -1306415132i32,
    #[strum(serialize = "AppliancePlantGeneticAnalyzer")]
    #[strum(
        props(
            name = "Plant Genetic Analyzer",
            desc = "The Genetic Analyzer can be used to process samples from the <link=ThingItemPlantSampler><color=green>Plant Sampler</color></link>. Once processed, the genetic information of the sampled plant can be viewed by clicking on the search button.\n\nIndividual Gene Value Widgets: \nMost gene values will appear as a sliding bar between a minimum value on the left and a maximum value on the right. The actual value of the gene is in the middle of the bar, in orange.\n\nMultiple Gene Value Widgets: \nFor temperature and pressure ranges, four genes appear on the same widget. The orange values underneath the bar are the minimum and maximum thresholds for growth. Outside of this range, the plant will stop growing and eventually die. The blue values underneath the bar are the minimum and maximum thresholds for ideal growth. Inside of this range, the plant will grow at maximum speed. The white values above the bar are the minimum and maximum achievable values for the growth threshold.",
            value = "-1303038067"
        )
    )]
    AppliancePlantGeneticAnalyzer = -1303038067i32,
    #[strum(serialize = "ItemIronIngot")]
    #[strum(
        props(
            name = "Ingot (Iron)",
            desc = "The most basic unit of construction available to <link=Stationeers><color=#0080FFFF>Stationeer</color></link>-kind, iron ingots are created by smelting <link=ThingItemIronOre><color=green>Ore (Iron)</color></link> in the <link=ThingStructureFurnace><color=green>Furnace</color></link> and <link=ThingStructureArcFurnace><color=green>Arc Furnace</color></link>, and used to create a variety of items.",
            value = "-1301215609"
        )
    )]
    ItemIronIngot = -1301215609i32,
    #[strum(serialize = "StructureSleeperVertical")]
    #[strum(
        props(
            name = "Sleeper Vertical",
            desc = "The vertical variant of the sleeper. Will keep players hydrated and fed while they are logged out - as long as a breathable atmosphere is provided.",
            value = "-1300059018"
        )
    )]
    StructureSleeperVertical = -1300059018i32,
    #[strum(serialize = "Landingpad_2x2CenterPiece01")]
    #[strum(
        props(
            name = "Landingpad 2x2 Center Piece",
            desc = "Recommended for larger traders. This allows for the creation of 4x4 and 6x6 landing areas with symetrical doors",
            value = "-1295222317"
        )
    )]
    Landingpad2X2CenterPiece01 = -1295222317i32,
    #[strum(serialize = "SeedBag_Corn")]
    #[strum(
        props(
            name = "Corn Seeds",
            desc = "Grow a <link=ThingItemCorn><color=green>Corn</color></link>.",
            value = "-1290755415"
        )
    )]
    SeedBagCorn = -1290755415i32,
    #[strum(serialize = "StructureDigitalValve")]
    #[strum(
        props(
            name = "Digital Valve",
            desc = "The digital valve allows <link=Stationeers><color=#0080FFFF>Stationeers</color></link> to create <link=LogicPage><color=#0080FFFF>logic-controlled</color></link> valves and pipe networks.",
            value = "-1280984102"
        )
    )]
    StructureDigitalValve = -1280984102i32,
    #[strum(serialize = "StructureTankConnector")]
    #[strum(
        props(
            name = "Tank Connector",
            desc = "Tank connectors are basic mounting devices that allow you to attach a <link=ThingDynamicGasCanisterEmpty><color=green>Portable Gas Tank</color></link> to a gas pipe network.",
            value = "-1276379454"
        )
    )]
    StructureTankConnector = -1276379454i32,
    #[strum(serialize = "ItemSuitModCryogenicUpgrade")]
    #[strum(
        props(
            name = "Cryogenic Suit Upgrade",
            desc = "Enables suits with basic cooling functionality to work with cryogenic liquid.",
            value = "-1274308304"
        )
    )]
    ItemSuitModCryogenicUpgrade = -1274308304i32,
    #[strum(serialize = "ItemKitLandingPadWaypoint")]
    #[strum(props(name = "Kit (Landing Pad Runway)", desc = "", value = "-1267511065"))]
    ItemKitLandingPadWaypoint = -1267511065i32,
    #[strum(serialize = "DynamicGasTankAdvancedOxygen")]
    #[strum(
        props(
            name = "Portable Gas Tank Mk II (Oxygen)",
            desc = "0.Mode0\n1.Mode1",
            value = "-1264455519"
        )
    )]
    DynamicGasTankAdvancedOxygen = -1264455519i32,
    #[strum(serialize = "ItemBasketBall")]
    #[strum(props(name = "Basket Ball", desc = "", value = "-1262580790"))]
    ItemBasketBall = -1262580790i32,
    #[strum(serialize = "ItemSpacepack")]
    #[strum(
        props(
            name = "Spacepack",
            desc = "The basic <link=CHAC><color=#0080FFFF>CHAC</color></link> spacepack isn't 'technically' a jetpack, it's a gas thruster. It can be powered by any gas, so long as the internal pressure of the <link=AtmosphericsPage><color=#0080FFFF>canister</color></link> is higher than the ambient external pressure. If the external pressure is greater, the spacepack will not function.\nIndispensable for building, mining and general movement, it has ten storage slots and lets <link=Stationeers><color=#0080FFFF>Stationeers</color></link> fly at 3m/s, compared to the more powerful <link=ThingItemJetpackBasic><color=green>Jetpack Basic</color></link> or <link=ThingItemHardJetpack><color=green>Hardsuit Jetpack</color></link>. Adjusting the thrust value alters your rate of acceleration, while activating the stablizer causes the spacepack to hover when a given height is reached.\nUSE: 'J' to activate; 'space' to fly up; 'left ctrl' to descend; and 'WASD' to move.",
            value = "-1260618380"
        )
    )]
    ItemSpacepack = -1260618380i32,
    #[strum(serialize = "ItemKitRocketDatalink")]
    #[strum(props(name = "Kit (Rocket Datalink)", desc = "", value = "-1256996603"))]
    ItemKitRocketDatalink = -1256996603i32,
    #[strum(serialize = "StructureGasSensor")]
    #[strum(
        props(
            name = "Gas Sensor",
            desc = "Gas sensors are designed to monitor and report basic atmospheric information, including temperature, pressure, and gas ratios. They also make wonderful wedding presents.",
            value = "-1252983604"
        )
    )]
    StructureGasSensor = -1252983604i32,
    #[strum(serialize = "ItemPureIceCarbonDioxide")]
    #[strum(
        props(
            name = "Pure Ice Carbon Dioxide",
            desc = "A frozen chunk of pure <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link>",
            value = "-1251009404"
        )
    )]
    ItemPureIceCarbonDioxide = -1251009404i32,
    #[strum(serialize = "ItemKitTurboVolumePump")]
    #[strum(
        props(name = "Kit (Turbo Volume Pump - Gas)", desc = "", value = "-1248429712")
    )]
    ItemKitTurboVolumePump = -1248429712i32,
    #[strum(serialize = "ItemGasFilterNitrousOxide")]
    #[strum(props(name = "Filter (Nitrous Oxide)", desc = "", value = "-1247674305"))]
    ItemGasFilterNitrousOxide = -1247674305i32,
    #[strum(serialize = "StructureChairThickDouble")]
    #[strum(props(name = "Chair (Thick Double)", desc = "", value = "-1245724402"))]
    StructureChairThickDouble = -1245724402i32,
    #[strum(serialize = "StructureWallPaddingArchVent")]
    #[strum(props(name = "Wall (Padding Arch Vent)", desc = "", value = "-1243329828"))]
    StructureWallPaddingArchVent = -1243329828i32,
    #[strum(serialize = "ItemKitConsole")]
    #[strum(props(name = "Kit (Consoles)", desc = "", value = "-1241851179"))]
    ItemKitConsole = -1241851179i32,
    #[strum(serialize = "ItemKitBeds")]
    #[strum(props(name = "Kit (Beds)", desc = "", value = "-1241256797"))]
    ItemKitBeds = -1241256797i32,
    #[strum(serialize = "StructureFrameIron")]
    #[strum(props(name = "Iron Frame", desc = "", value = "-1240951678"))]
    StructureFrameIron = -1240951678i32,
    #[strum(serialize = "ItemDirtyOre")]
    #[strum(
        props(
            name = "Dirty Ore",
            desc = "Ore mined from bedrock via the <link=ThingStructureDeepMiner><color=green>Deep Miner</color></link> which then can be used in the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link>, or <link=ThingStructureCombustionCentrifuge><color=green>Combustion Centrifuge</color></link>. Once processed, it produces ore in a ratio similar to the average found on the planet's surface. ",
            value = "-1234745580"
        )
    )]
    ItemDirtyOre = -1234745580i32,
    #[strum(serialize = "StructureLargeDirectHeatExchangeGastoGas")]
    #[strum(
        props(
            name = "Large Direct Heat Exchanger - Gas + Gas",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "-1230658883"
        )
    )]
    StructureLargeDirectHeatExchangeGastoGas = -1230658883i32,
    #[strum(serialize = "ItemSensorProcessingUnitOreScanner")]
    #[strum(
        props(
            name = "Sensor Processing Unit (Ore Scanner)",
            desc = "The Sensor Processing unit can be inserted into <link=ThingItemSensorLenses><color=green>Sensor Lenses</color></link> to reveal underground minerals in a HUD.",
            value = "-1219128491"
        )
    )]
    ItemSensorProcessingUnitOreScanner = -1219128491i32,
    #[strum(serialize = "StructurePictureFrameThickPortraitSmall")]
    #[strum(
        props(
            name = "Picture Frame Thick Portrait Small",
            desc = "",
            value = "-1218579821"
        )
    )]
    StructurePictureFrameThickPortraitSmall = -1218579821i32,
    #[strum(serialize = "ItemGasFilterOxygenL")]
    #[strum(props(name = "Heavy Filter (Oxygen)", desc = "", value = "-1217998945"))]
    ItemGasFilterOxygenL = -1217998945i32,
    #[strum(serialize = "Landingpad_LiquidConnectorInwardPiece")]
    #[strum(props(name = "Landingpad Liquid Input", desc = "", value = "-1216167727"))]
    LandingpadLiquidConnectorInwardPiece = -1216167727i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation008")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "-1214467897"
        )
    )]
    ItemWreckageStructureWeatherStation008 = -1214467897i32,
    #[strum(serialize = "ItemPlantThermogenic_Creative")]
    #[strum(
        props(name = "Thermogenic Plant Creative", desc = "", value = "-1208890208")
    )]
    ItemPlantThermogenicCreative = -1208890208i32,
    #[strum(serialize = "ItemRocketScanningHead")]
    #[strum(props(name = "Rocket Scanner Head", desc = "", value = "-1198702771"))]
    ItemRocketScanningHead = -1198702771i32,
    #[strum(serialize = "StructureCableStraightBurnt")]
    #[strum(props(name = "Burnt Cable (Straight)", desc = "", value = "-1196981113"))]
    StructureCableStraightBurnt = -1196981113i32,
    #[strum(serialize = "ItemHydroponicTray")]
    #[strum(
        props(
            name = "Kit (Hydroponic Tray)",
            desc = "This kits creates a <link=ThingStructureHydroponicsTray><color=green>Hydroponics Tray</color></link> for growing various plants.",
            value = "-1193543727"
        )
    )]
    ItemHydroponicTray = -1193543727i32,
    #[strum(serialize = "ItemCannedRicePudding")]
    #[strum(
        props(
            name = "Canned Rice Pudding",
            desc = "Made in an <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link> or <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link>, using <link=ThingItemCookedRice><color=green>Cooked Rice</color></link> and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link>, canned rice pudding is a sweet treat, fairly high in nutrition, and does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "-1185552595"
        )
    )]
    ItemCannedRicePudding = -1185552595i32,
    #[strum(serialize = "StructureInLineTankLiquid1x2")]
    #[strum(
        props(
            name = "In-Line Tank Liquid",
            desc = "A small expansion tank that increases the volume of a pipe network.",
            value = "-1183969663"
        )
    )]
    StructureInLineTankLiquid1X2 = -1183969663i32,
    #[strum(serialize = "StructureInteriorDoorTriangle")]
    #[strum(
        props(
            name = "Interior Door Triangle",
            desc = "0.Operate\n1.Logic",
            value = "-1182923101"
        )
    )]
    StructureInteriorDoorTriangle = -1182923101i32,
    #[strum(serialize = "ItemKitElectronicsPrinter")]
    #[strum(props(name = "Kit (Electronics Printer)", desc = "", value = "-1181922382"))]
    ItemKitElectronicsPrinter = -1181922382i32,
    #[strum(serialize = "StructureWaterBottleFiller")]
    #[strum(props(name = "Water Bottle Filler", desc = "", value = "-1178961954"))]
    StructureWaterBottleFiller = -1178961954i32,
    #[strum(serialize = "StructureWallVent")]
    #[strum(
        props(
            name = "Wall Vent",
            desc = "Used to mix atmospheres passively between two walls.",
            value = "-1177469307"
        )
    )]
    StructureWallVent = -1177469307i32,
    #[strum(serialize = "ItemSensorLenses")]
    #[strum(
        props(
            name = "Sensor Lenses",
            desc = "These <link=Norsec><color=#0080FFFF>Norsec</color></link> glasses might not be the most fashionable thing, but when a <link=ThingItemSensorProcessingUnitOreScanner><color=green>Sensor Processing Unit (Ore Scanner)</color></link> is inserted, Stationeers can use these handy glasses to x-ray the ground and find ores that are hidden beneath the surface.",
            value = "-1176140051"
        )
    )]
    ItemSensorLenses = -1176140051i32,
    #[strum(serialize = "ItemSoundCartridgeLeads")]
    #[strum(props(name = "Sound Cartridge Leads", desc = "", value = "-1174735962"))]
    ItemSoundCartridgeLeads = -1174735962i32,
    #[strum(serialize = "StructureMediumConvectionRadiatorLiquid")]
    #[strum(
        props(
            name = "Medium Convection Radiator Liquid",
            desc = "A stand-alone liquid radiator unit optimized for exchanging heat with its surrounding atmosphere.",
            value = "-1169014183"
        )
    )]
    StructureMediumConvectionRadiatorLiquid = -1169014183i32,
    #[strum(serialize = "ItemKitFridgeBig")]
    #[strum(props(name = "Kit (Fridge Large)", desc = "", value = "-1168199498"))]
    ItemKitFridgeBig = -1168199498i32,
    #[strum(serialize = "ItemKitPipeLiquid")]
    #[strum(props(name = "Kit (Liquid Pipe)", desc = "", value = "-1166461357"))]
    ItemKitPipeLiquid = -1166461357i32,
    #[strum(serialize = "StructureWallFlatCornerTriangleFlat")]
    #[strum(
        props(
            name = "Wall (Flat Corner Triangle Flat)",
            desc = "",
            value = "-1161662836"
        )
    )]
    StructureWallFlatCornerTriangleFlat = -1161662836i32,
    #[strum(serialize = "StructureLogicMathUnary")]
    #[strum(
        props(
            name = "Math Unary",
            desc = "0.Ceil\n1.Floor\n2.Abs\n3.Log\n4.Exp\n5.Round\n6.Rand\n7.Sqrt\n8.Sin\n9.Cos\n10.Tan\n11.Asin\n12.Acos\n13.Atan\n14.Not",
            value = "-1160020195"
        )
    )]
    StructureLogicMathUnary = -1160020195i32,
    #[strum(serialize = "ItemPlantEndothermic_Creative")]
    #[strum(
        props(name = "Endothermic Plant Creative", desc = "", value = "-1159179557")
    )]
    ItemPlantEndothermicCreative = -1159179557i32,
    #[strum(serialize = "ItemSensorProcessingUnitCelestialScanner")]
    #[strum(
        props(
            name = "Sensor Processing Unit (Celestial Scanner)",
            desc = "",
            value = "-1154200014"
        )
    )]
    ItemSensorProcessingUnitCelestialScanner = -1154200014i32,
    #[strum(serialize = "StructureChairRectangleDouble")]
    #[strum(props(name = "Chair (Rectangle Double)", desc = "", value = "-1152812099"))]
    StructureChairRectangleDouble = -1152812099i32,
    #[strum(serialize = "ItemGasCanisterOxygen")]
    #[strum(props(name = "Canister (Oxygen)", desc = "", value = "-1152261938"))]
    ItemGasCanisterOxygen = -1152261938i32,
    #[strum(serialize = "ItemPureIceOxygen")]
    #[strum(
        props(
            name = "Pure Ice Oxygen",
            desc = "A frozen chunk of pure <link=GasOxygen><color=#44AD83>Oxygen</color></link>",
            value = "-1150448260"
        )
    )]
    ItemPureIceOxygen = -1150448260i32,
    #[strum(serialize = "StructureBackPressureRegulator")]
    #[strum(
        props(
            name = "Back Pressure Regulator",
            desc = "Unlike the <link=ThingStructurePressureRegulator><color=green>Pressure Regulator</color></link>, which closes when the input exceeds a given pressure, the back pressure regulator opens when input pressure reaches a given value.",
            value = "-1149857558"
        )
    )]
    StructureBackPressureRegulator = -1149857558i32,
    #[strum(serialize = "StructurePictureFrameThinMountLandscapeLarge")]
    #[strum(
        props(
            name = "Picture Frame Thin Landscape Large",
            desc = "",
            value = "-1146760430"
        )
    )]
    StructurePictureFrameThinMountLandscapeLarge = -1146760430i32,
    #[strum(serialize = "StructureMediumRadiatorLiquid")]
    #[strum(
        props(
            name = "Medium Radiator Liquid",
            desc = "A stand-alone liquid radiator unit optimized for radiating heat in vacuums.",
            value = "-1141760613"
        )
    )]
    StructureMediumRadiatorLiquid = -1141760613i32,
    #[strum(serialize = "ApplianceMicrowave")]
    #[strum(
        props(
            name = "Microwave",
            desc = "While countless 'better' ways of cooking <link=FoodPage><color=#0080FFFF>Food</color></link> have been invented in the last few hundred years, few are as durable or easy to fabricate as the OK-Zoomer microwave. Licensed from <link=Xigo><color=#0080FFFF>Xigo</color></link>, the plans are based on a classic model from the mid-21st century, giving it a charmingly retro feel. But don't worry, it oscillates <link=GasWater><color=#44AD83>Water</color></link> molecules more than adequately. \nJust bolt it to a <link=ThingStructureBench><color=green>Powered Bench</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link> to power it, follow the recipe, and you're cooking.",
            value = "-1136173965"
        )
    )]
    ApplianceMicrowave = -1136173965i32,
    #[strum(serialize = "ItemPipeGasMixer")]
    #[strum(
        props(
            name = "Kit (Gas Mixer)",
            desc = "This kit creates a <link=ThingStructureGasMixer><color=green>Gas Mixer</color></link>.",
            value = "-1134459463"
        )
    )]
    ItemPipeGasMixer = -1134459463i32,
    #[strum(serialize = "CircuitboardModeControl")]
    #[strum(
        props(
            name = "Mode Control",
            desc = "Can't decide which mode you love most? This circuit board allows you to switch any connected device between operation modes.",
            value = "-1134148135"
        )
    )]
    CircuitboardModeControl = -1134148135i32,
    #[strum(serialize = "StructureActiveVent")]
    #[strum(
        props(
            name = "Active Vent",
            desc = "The active vent is a powered device for maintaining <link=GasPage><color=#0080FFFF>gas</color></link> pressure by pumping gas into (or out of) a pipe network. The vent has two modes: 'Outward' sets it to pump gas into a space until pressure is reached; 'Inward' sets it to pump gas out until pressure is reached. The pressure parameter can be set on a connected <link=ThingStructureConsole><color=green>Console</color></link>. Default pressure is 101kPa for Outward; 0kPa for Inward ...",
            value = "-1129453144"
        )
    )]
    StructureActiveVent = -1129453144i32,
    #[strum(serialize = "StructureWallPaddedArchCorner")]
    #[strum(props(name = "Wall (Padded Arch Corner)", desc = "", value = "-1126688298"))]
    StructureWallPaddedArchCorner = -1126688298i32,
    #[strum(serialize = "StructurePlanter")]
    #[strum(
        props(
            name = "Planter",
            desc = "A small planter for decorative or hydroponic purposes. Can be connected to <link=GasWater><color=#44AD83>Water</color></link>, or watered manually using a <link=ThingItemWaterBottle><color=green>Water Bottle</color></link> or <link=ThingItemGasCanisterWater><color=green>Liquid Canister (Water)</color></link>.",
            value = "-1125641329"
        )
    )]
    StructurePlanter = -1125641329i32,
    #[strum(serialize = "StructureBatteryMedium")]
    #[strum(
        props(
            name = "Battery (Medium)",
            desc = "0.Empty\n1.Critical\n2.VeryLow\n3.Low\n4.Medium\n5.High\n6.Full",
            value = "-1125305264"
        )
    )]
    StructureBatteryMedium = -1125305264i32,
    #[strum(serialize = "ItemHorticultureBelt")]
    #[strum(props(name = "Horticulture Belt", desc = "", value = "-1117581553"))]
    ItemHorticultureBelt = -1117581553i32,
    #[strum(serialize = "CartridgeMedicalAnalyser")]
    #[strum(
        props(
            name = "Medical Analyzer",
            desc = "When added to the OreCore <link=ThingItemTablet><color=green>Handheld Tablet</color></link>, <link=Asura><color=#0080FFFF>Asura's</color></link>'s ReadyMed medical analyzer reveals the health, or otherwise, of users various organs. Due to a design flaw, older models were notorious for producing quasar-like levels of x-ray radiation. Recent advances in shielding have more than halved the risk to users.",
            value = "-1116110181"
        )
    )]
    CartridgeMedicalAnalyser = -1116110181i32,
    #[strum(serialize = "StructureCompositeFloorGrating3")]
    #[strum(
        props(
            name = "Composite Floor Grating  (Type 3)",
            desc = "",
            value = "-1113471627"
        )
    )]
    StructureCompositeFloorGrating3 = -1113471627i32,
    #[strum(serialize = "ItemPlainCake")]
    #[strum(props(name = "Cake", desc = "", value = "-1108244510"))]
    ItemPlainCake = -1108244510i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation004")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "-1104478996"
        )
    )]
    ItemWreckageStructureWeatherStation004 = -1104478996i32,
    #[strum(serialize = "StructureCableFuse1k")]
    #[strum(props(name = "Fuse (1kW)", desc = "", value = "-1103727120"))]
    StructureCableFuse1K = -1103727120i32,
    #[strum(serialize = "WeaponTorpedo")]
    #[strum(props(name = "Torpedo", desc = "", value = "-1102977898"))]
    WeaponTorpedo = -1102977898i32,
    #[strum(serialize = "StructureWallPaddingThin")]
    #[strum(props(name = "Wall (Padding Thin)", desc = "", value = "-1102403554"))]
    StructureWallPaddingThin = -1102403554i32,
    #[strum(serialize = "Landingpad_GasConnectorOutwardPiece")]
    #[strum(
        props(
            name = "Landingpad Gas Output",
            desc = "Pumps gas purchased from a trader out of the landing pad. You can increase the landing pad's gas storage capacity by adding more <link=ThingLandingpad_GasCylinderTankPiece><color=green>Landingpad Gas Storage</color></link> to the landing pad.",
            value = "-1100218307"
        )
    )]
    LandingpadGasConnectorOutwardPiece = -1100218307i32,
    #[strum(serialize = "AppliancePlantGeneticSplicer")]
    #[strum(
        props(
            name = "Plant Genetic Splicer",
            desc = "The Genetic Splicer can be used to copy a single <link=GeneticsPage><color=#0080FFFF>gene</color></link> from one 'source' plant to another 'target' plant of the same type. After copying, the source plant will be destroyed.\n        \nTo begin splicing, place a plant or seed bag in the left slot (source) and place another plant or seed bag of the same type in the right slot (target). You can select a gene using the arrow buttons. Close the sliding door and press the green activate button. Once splicing has begun, the device will be locked until the process has finished (which will take approximately twenty minutes). If you want to cancel splicing you can power off the bench or detach the appliance as a last resort.",
            value = "-1094868323"
        )
    )]
    AppliancePlantGeneticSplicer = -1094868323i32,
    #[strum(serialize = "StructureMediumRocketGasFuelTank")]
    #[strum(props(name = "Gas Capsule Tank Medium", desc = "", value = "-1093860567"))]
    StructureMediumRocketGasFuelTank = -1093860567i32,
    #[strum(serialize = "StructureStairs4x2Rails")]
    #[strum(props(name = "Stairs with Rails", desc = "", value = "-1088008720"))]
    StructureStairs4X2Rails = -1088008720i32,
    #[strum(serialize = "StructureShowerPowered")]
    #[strum(props(name = "Shower (Powered)", desc = "", value = "-1081797501"))]
    StructureShowerPowered = -1081797501i32,
    #[strum(serialize = "ItemCookedMushroom")]
    #[strum(
        props(
            name = "Cooked Mushroom",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "-1076892658"
        )
    )]
    ItemCookedMushroom = -1076892658i32,
    #[strum(serialize = "ItemGlasses")]
    #[strum(props(name = "Glasses", desc = "", value = "-1068925231"))]
    ItemGlasses = -1068925231i32,
    #[strum(serialize = "KitchenTableSimpleTall")]
    #[strum(
        props(name = "Kitchen Table (Simple Tall)", desc = "", value = "-1068629349")
    )]
    KitchenTableSimpleTall = -1068629349i32,
    #[strum(serialize = "ItemGasFilterOxygenM")]
    #[strum(props(name = "Medium Filter (Oxygen)", desc = "", value = "-1067319543"))]
    ItemGasFilterOxygenM = -1067319543i32,
    #[strum(serialize = "StructureTransformerMedium")]
    #[strum(
        props(
            name = "Transformer (Medium)",
            desc = "Transformers control the maximum power that will flow down a sub-network of cables, to prevent overloading <link=ElectronicPage><color=#0080FFFF>electrical</color></link> systems. \nMedium transformers are used in larger setups where more than 5000W is required, with output that can be set to a maximum of 25000W.\nNote that transformers also operate as data isolators, preventing data flowing into any network beyond it.",
            value = "-1065725831"
        )
    )]
    StructureTransformerMedium = -1065725831i32,
    #[strum(serialize = "ItemKitDynamicCanister")]
    #[strum(props(name = "Kit (Portable Gas Tank)", desc = "", value = "-1061945368"))]
    ItemKitDynamicCanister = -1061945368i32,
    #[strum(serialize = "ItemEmergencyPickaxe")]
    #[strum(props(name = "Emergency Pickaxe", desc = "", value = "-1061510408"))]
    ItemEmergencyPickaxe = -1061510408i32,
    #[strum(serialize = "ItemWheat")]
    #[strum(
        props(
            name = "Wheat",
            desc = "A classical symbol of growth and new life, wheat takes a moderate time to grow. Its main use is to create <link=ReagentPage><color=#0080FFFF>flour</color></link> using the <link=ThingApplianceReagentProcessor><color=green>Reagent Processor</color></link>.",
            value = "-1057658015"
        )
    )]
    ItemWheat = -1057658015i32,
    #[strum(serialize = "ItemEmergencyArcWelder")]
    #[strum(props(name = "Emergency Arc Welder", desc = "", value = "-1056029600"))]
    ItemEmergencyArcWelder = -1056029600i32,
    #[strum(serialize = "ItemGasFilterOxygenInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Oxygen)",
            desc = "A filter that selectively targets Oxygen. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-1055451111"
        )
    )]
    ItemGasFilterOxygenInfinite = -1055451111i32,
    #[strum(serialize = "StructureLiquidTurboVolumePump")]
    #[strum(
        props(
            name = "Turbo Volume Pump (Liquid)",
            desc = "Shifts 10 times more liquid than a basic <link=ThingStructureVolumePump><color=green>Volume Pump</color></link>, with a mode that can be set to flow in either direction.",
            value = "-1051805505"
        )
    )]
    StructureLiquidTurboVolumePump = -1051805505i32,
    #[strum(serialize = "ItemPureIceLiquidHydrogen")]
    #[strum(
        props(
            name = "Pure Ice Liquid Hydrogen",
            desc = "A frozen chunk of pure <link=GasLiquidHydrogen><color=#44AD83>Liquid Hydrogen</color></link>",
            value = "-1044933269"
        )
    )]
    ItemPureIceLiquidHydrogen = -1044933269i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerInnerLongR")]
    #[strum(
        props(
            name = "Composite Cladding (Angled Corner Inner Long R)",
            desc = "",
            value = "-1032590967"
        )
    )]
    StructureCompositeCladdingAngledCornerInnerLongR = -1032590967i32,
    #[strum(serialize = "StructureAreaPowerControlReversed")]
    #[strum(
        props(
            name = "Area Power Control",
            desc = "An Area Power Control (APC) has three main functions. \nIts primary purpose is to regulate power flow, ensuring uninterrupted performance from devices and machinery, especially those with a fluctuating draw. \nAPCs also create sub-networks, as no devices on the far side of an APC are visible on the main network. \nLastly, an APC charges batteries, which can provide backup power to the sub-network in the case of an outage. Note that an APC requires a battery to stabilize power draw. It also has two variants, each allowing power to flow in one direction only.",
            value = "-1032513487"
        )
    )]
    StructureAreaPowerControlReversed = -1032513487i32,
    #[strum(serialize = "StructureChuteOutlet")]
    #[strum(
        props(
            name = "Chute Outlet",
            desc = "The aim for any <link=Stationeers><color=#0080FFFF>Stationeer</color></link> is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nThe chute outlet is an aperture for exiting items from <link=ImportExportPage><color=#0080FFFF>import/export</color></link> networks. Note that the outlet's origins in zero-gravity mining means they are permanently open, rather than interactable, but can be connected to <link=LogicUnitPage><color=#0080FFFF>logic</color></link> systems to monitor throughput.",
            value = "-1022714809"
        )
    )]
    StructureChuteOutlet = -1022714809i32,
    #[strum(serialize = "ItemKitHarvie")]
    #[strum(props(name = "Kit (Harvie)", desc = "", value = "-1022693454"))]
    ItemKitHarvie = -1022693454i32,
    #[strum(serialize = "ItemGasCanisterFuel")]
    #[strum(props(name = "Canister (Fuel)", desc = "", value = "-1014695176"))]
    ItemGasCanisterFuel = -1014695176i32,
    #[strum(serialize = "StructureCompositeWall04")]
    #[strum(props(name = "Composite Wall (Type 4)", desc = "", value = "-1011701267"))]
    StructureCompositeWall04 = -1011701267i32,
    #[strum(serialize = "StructureSorter")]
    #[strum(
        props(
            name = "Sorter",
            desc = "No amount of automation is complete without some way of moving different items to different parts of a system. The <link=Xigo><color=#0080FFFF>Xigo</color></link> A2B sorter can be programmed via a computer with a <link=ThingMotherboardSorter><color=green>Sorter Motherboard</color></link> to direct various items into different chute networks. Filtered items are always passed out the righthand side of the sorter, while non filtered items continue straight through.",
            value = "-1009150565"
        )
    )]
    StructureSorter = -1009150565i32,
    #[strum(serialize = "StructurePipeLabel")]
    #[strum(
        props(
            name = "Pipe Label",
            desc = "As its perspicacious name suggests, the pipe label is designed to be attached to a straight stretch of pipe. Users can then label the label with the <link=ThingItemLabeller><color=green>Labeller</color></link>.",
            value = "-999721119"
        )
    )]
    StructurePipeLabel = -999721119i32,
    #[strum(serialize = "ItemCannedEdamame")]
    #[strum(
        props(
            name = "Canned Edamame",
            desc = "Made in an <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link> or <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link>, using <link=ThingItemCookedSoybean><color=green>Cooked Soybean</color></link> and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link>, canned edamame beans are fairly high in nutrition, and do not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "-999714082"
        )
    )]
    ItemCannedEdamame = -999714082i32,
    #[strum(serialize = "ItemTomato")]
    #[strum(
        props(
            name = "Tomato",
            desc = "Tomato plants are perennial, and will produce multiple harvests without needing to be replanted. Once the plant is mature, it will fruit at a moderate pace.",
            value = "-998592080"
        )
    )]
    ItemTomato = -998592080i32,
    #[strum(serialize = "ItemCobaltOre")]
    #[strum(
        props(
            name = "Ore (Cobalt)",
            desc = "Cobalt is a chemical element with the symbol \"Co\" and is typically found in only small deposits. Cobalt is a rare substance, but used create the <link=ThingItemHealPill><color=green>Heal Pill</color></link> and several  <link=IngotPage><color=#0080FFFF>alloys</color></link>.",
            value = "-983091249"
        )
    )]
    ItemCobaltOre = -983091249i32,
    #[strum(serialize = "StructureCableCorner4HBurnt")]
    #[strum(
        props(name = "Burnt Heavy Cable (4-Way Corner)", desc = "", value = "-981223316")
    )]
    StructureCableCorner4HBurnt = -981223316i32,
    #[strum(serialize = "Landingpad_StraightPiece01")]
    #[strum(
        props(
            name = "Landingpad Straight",
            desc = "Extends the size of the landing pad area. A basic trader shuttle requires a 3x3 clear landing area.",
            value = "-976273247"
        )
    )]
    LandingpadStraightPiece01 = -976273247i32,
    #[strum(serialize = "StructureMediumRadiator")]
    #[strum(
        props(
            name = "Medium Radiator",
            desc = "A stand-alone radiator unit optimized for radiating heat in vacuums.",
            value = "-975966237"
        )
    )]
    StructureMediumRadiator = -975966237i32,
    #[strum(serialize = "ItemDynamicScrubber")]
    #[strum(props(name = "Kit (Portable Scrubber)", desc = "", value = "-971920158"))]
    ItemDynamicScrubber = -971920158i32,
    #[strum(serialize = "StructureCondensationValve")]
    #[strum(
        props(
            name = "Condensation Valve",
            desc = "Allows for the removal of any liquids from a gas pipe into a liquid pipe. Only allows liquids to pass in one direction.",
            value = "-965741795"
        )
    )]
    StructureCondensationValve = -965741795i32,
    #[strum(serialize = "StructureChuteUmbilicalMale")]
    #[strum(
        props(
            name = "Umbilical (Chute)",
            desc = "0.Left\n1.Center\n2.Right",
            value = "-958884053"
        )
    )]
    StructureChuteUmbilicalMale = -958884053i32,
    #[strum(serialize = "ItemKitElevator")]
    #[strum(props(name = "Kit (Elevator)", desc = "", value = "-945806652"))]
    ItemKitElevator = -945806652i32,
    #[strum(serialize = "StructureSolarPanelReinforced")]
    #[strum(
        props(
            name = "Solar Panel (Heavy)",
            desc = "This solar panel is resistant to storm damage.",
            value = "-934345724"
        )
    )]
    StructureSolarPanelReinforced = -934345724i32,
    #[strum(serialize = "ItemKitRocketTransformerSmall")]
    #[strum(
        props(name = "Kit (Transformer Small (Rocket))", desc = "", value = "-932335800")
    )]
    ItemKitRocketTransformerSmall = -932335800i32,
    #[strum(serialize = "CartridgeConfiguration")]
    #[strum(props(name = "Configuration", desc = "", value = "-932136011"))]
    CartridgeConfiguration = -932136011i32,
    #[strum(serialize = "ItemSilverIngot")]
    #[strum(props(name = "Ingot (Silver)", desc = "", value = "-929742000"))]
    ItemSilverIngot = -929742000i32,
    #[strum(serialize = "ItemKitHydroponicAutomated")]
    #[strum(
        props(name = "Kit (Automated Hydroponics)", desc = "", value = "-927931558")
    )]
    ItemKitHydroponicAutomated = -927931558i32,
    #[strum(serialize = "StructureSmallTableRectangleSingle")]
    #[strum(
        props(name = "Small (Table Rectangle Single)", desc = "", value = "-924678969")
    )]
    StructureSmallTableRectangleSingle = -924678969i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation005")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "-919745414"
        )
    )]
    ItemWreckageStructureWeatherStation005 = -919745414i32,
    #[strum(serialize = "ItemSilverOre")]
    #[strum(
        props(
            name = "Ore (Silver)",
            desc = "Silver is a chemical element with the symbol \"Ag\". Valued by many <link=Stationeers><color=#0080FFFF>Stationeers</color></link> for its attractive luster and sheen, it is also used in a variety of <link=ElectronicPage><color=#0080FFFF>electronics</color></link> components and <link=IngotPage><color=#0080FFFF>alloys</color></link>.",
            value = "-916518678"
        )
    )]
    ItemSilverOre = -916518678i32,
    #[strum(serialize = "StructurePipeTJunction")]
    #[strum(
        props(
            name = "Pipe (T Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeTJunction><color=green>Insulated Pipe (T Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-913817472"
        )
    )]
    StructurePipeTJunction = -913817472i32,
    #[strum(serialize = "ItemPickaxe")]
    #[strum(
        props(
            name = "Pickaxe",
            desc = "When the sun sets and the <link=ThingItemMiningDrill><color=green>Mining Drill</color></link> runs dead, its batteries drained and your <link=ThingStructureSolarPanel><color=green>Solar Panel</color></link> cold and lifeless, the <link=ThingStructureAutolathe><color=green>Autolathe</color></link> empty, the way forward unclear, one thing holds back the endless night of defeat: the trusty pickaxe.",
            value = "-913649823"
        )
    )]
    ItemPickaxe = -913649823i32,
    #[strum(serialize = "ItemPipeLiquidRadiator")]
    #[strum(
        props(
            name = "Kit (Liquid Radiator)",
            desc = "This kit creates a <link=ThingStructureLiquidPipeRadiator><color=green>Liquid Pipe Convection Radiator</color></link>.",
            value = "-906521320"
        )
    )]
    ItemPipeLiquidRadiator = -906521320i32,
    #[strum(serialize = "StructurePortablesConnector")]
    #[strum(props(name = "Portables Connector", desc = "", value = "-899013427"))]
    StructurePortablesConnector = -899013427i32,
    #[strum(serialize = "StructureCompositeFloorGrating2")]
    #[strum(
        props(
            name = "Composite Floor Grating  (Type 2)",
            desc = "",
            value = "-895027741"
        )
    )]
    StructureCompositeFloorGrating2 = -895027741i32,
    #[strum(serialize = "StructureTransformerSmall")]
    #[strum(
        props(
            name = "Transformer (Small)",
            desc = "Transformers control the maximum power that will flow down a cable subnetwork, to prevent overloading <link=ElectronicPage><color=#0080FFFF>electrical</color></link> systems. Output on small transformers can be set from 0 to 5000W.\nNote that transformers operate as data isolators, preventing data flowing into any network beyond it.",
            value = "-890946730"
        )
    )]
    StructureTransformerSmall = -890946730i32,
    #[strum(serialize = "StructureCableCorner")]
    #[strum(
        props(
            name = "Cable (Corner)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life - so essential, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link> during the 3rd Decannual Stationeer Solar Conference.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "-889269388"
        )
    )]
    StructureCableCorner = -889269388i32,
    #[strum(serialize = "ItemKitChuteUmbilical")]
    #[strum(props(name = "Kit (Chute Umbilical)", desc = "", value = "-876560854"))]
    ItemKitChuteUmbilical = -876560854i32,
    #[strum(serialize = "ItemPureIceSteam")]
    #[strum(
        props(
            name = "Pure Ice Steam",
            desc = "A frozen chunk of pure <link=GasSteam><color=#44AD83>Steam</color></link>",
            value = "-874791066"
        )
    )]
    ItemPureIceSteam = -874791066i32,
    #[strum(serialize = "ItemBeacon")]
    #[strum(props(name = "Tracking Beacon", desc = "", value = "-869869491"))]
    ItemBeacon = -869869491i32,
    #[strum(serialize = "ItemKitWindTurbine")]
    #[strum(props(name = "Kit (Wind Turbine)", desc = "", value = "-868916503"))]
    ItemKitWindTurbine = -868916503i32,
    #[strum(serialize = "ItemKitRocketMiner")]
    #[strum(props(name = "Kit (Rocket Miner)", desc = "", value = "-867969909"))]
    ItemKitRocketMiner = -867969909i32,
    #[strum(serialize = "StructureStairwellBackPassthrough")]
    #[strum(
        props(name = "Stairwell (Back Passthrough)", desc = "", value = "-862048392")
    )]
    StructureStairwellBackPassthrough = -862048392i32,
    #[strum(serialize = "StructureWallArch")]
    #[strum(props(name = "Wall (Arch)", desc = "", value = "-858143148"))]
    StructureWallArch = -858143148i32,
    #[strum(serialize = "HumanSkull")]
    #[strum(props(name = "Human Skull", desc = "", value = "-857713709"))]
    HumanSkull = -857713709i32,
    #[strum(serialize = "StructureLogicMemory")]
    #[strum(props(name = "Logic Memory", desc = "", value = "-851746783"))]
    StructureLogicMemory = -851746783i32,
    #[strum(serialize = "StructureChuteBin")]
    #[strum(
        props(
            name = "Chute Bin",
            desc = "The <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> goal is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nLike most <link=Recurso><color=#0080FFFF>Recurso</color></link>-designed systems, chute bins are simple and robust powered items, allowing items to be manually passed into chute networks by pulling a lever. They can also be programmed with <link=LogicUnitPage><color=#0080FFFF>logic</color></link> to operate automatically, although full automation requires the use items such as a <link=ThingStructureSDBHopper><color=green>SDB Hopper</color></link>.",
            value = "-850484480"
        )
    )]
    StructureChuteBin = -850484480i32,
    #[strum(serialize = "ItemKitWallFlat")]
    #[strum(props(name = "Kit (Flat Wall)", desc = "", value = "-846838195"))]
    ItemKitWallFlat = -846838195i32,
    #[strum(serialize = "ItemActiveVent")]
    #[strum(
        props(
            name = "Kit (Active Vent)",
            desc = "When constructed, this kit places an <link=ThingStructureActiveVent><color=green>Active Vent</color></link> on any support structure.",
            value = "-842048328"
        )
    )]
    ItemActiveVent = -842048328i32,
    #[strum(serialize = "ItemFlashlight")]
    #[strum(
        props(
            name = "Flashlight",
            desc = "A flashlight with a narrow and wide beam options.",
            value = "-838472102"
        )
    )]
    ItemFlashlight = -838472102i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation001")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "-834664349"
        )
    )]
    ItemWreckageStructureWeatherStation001 = -834664349i32,
    #[strum(serialize = "ItemBiomass")]
    #[strum(
        props(
            name = "Biomass",
            desc = "Diced organic material that is returned when food and organic matter is passed through the <link=ThingStructureRecycler><color=green>Recycler</color></link> and <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link>. Can be burned in a <link=ThingStructureFurnace><color=green>Furnace</color></link> into <link=ThingItemCharcoal><color=green>Charcoal</color></link> for use in the <link=ThingStructureSolidFuelGenerator><color=green>Generator (Solid Fuel)</color></link>.",
            value = "-831480639"
        )
    )]
    ItemBiomass = -831480639i32,
    #[strum(serialize = "ItemKitPowerTransmitterOmni")]
    #[strum(
        props(name = "Kit (Power Transmitter Omni)", desc = "", value = "-831211676")
    )]
    ItemKitPowerTransmitterOmni = -831211676i32,
    #[strum(serialize = "StructureKlaxon")]
    #[strum(
        props(
            name = "Klaxon Speaker",
            desc = "Klaxons allow you to play over 50 announcements and sounds, depending on your <link=LogicPage><color=#0080FFFF>Logic</color></link> set-up. Set the mode to select the output.",
            value = "-828056979"
        )
    )]
    StructureKlaxon = -828056979i32,
    #[strum(serialize = "StructureElevatorLevelFront")]
    #[strum(props(name = "Elevator Level (Cabled)", desc = "", value = "-827912235"))]
    StructureElevatorLevelFront = -827912235i32,
    #[strum(serialize = "ItemKitPipeOrgan")]
    #[strum(props(name = "Kit (Pipe Organ)", desc = "", value = "-827125300"))]
    ItemKitPipeOrgan = -827125300i32,
    #[strum(serialize = "ItemKitWallPadded")]
    #[strum(props(name = "Kit (Padded Wall)", desc = "", value = "-821868990"))]
    ItemKitWallPadded = -821868990i32,
    #[strum(serialize = "DynamicGasCanisterFuel")]
    #[strum(
        props(
            name = "Portable Gas Tank (Fuel)",
            desc = "Portable tanks store gas. They're good at it. If you need to refill a tank, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or things get messy. You can refill a <link=ThingItemGasCanisterFuel><color=green>Canister (Fuel)</color></link> by attaching it to the tank's striped section. Or you could use a <link=ThingItemWrench><color=green>Wrench</color></link> to attach it to a rover or rocket for later. It's really up to you.",
            value = "-817051527"
        )
    )]
    DynamicGasCanisterFuel = -817051527i32,
    #[strum(serialize = "StructureReinforcedCompositeWindowSteel")]
    #[strum(
        props(
            name = "Reinforced Window (Composite Steel)",
            desc = "Enjoy vistas of even the most savage, alien landscapes with these heavy duty window frames, which are resistant to pressure differentials up to 1MPa.",
            value = "-816454272"
        )
    )]
    StructureReinforcedCompositeWindowSteel = -816454272i32,
    #[strum(serialize = "StructureConsoleLED5")]
    #[strum(
        props(
            name = "LED Display (Small)",
            desc = "0.Default\n1.Percent\n2.Power",
            value = "-815193061"
        )
    )]
    StructureConsoleLed5 = -815193061i32,
    #[strum(serialize = "StructureInsulatedInLineTankLiquid1x1")]
    #[strum(
        props(
            name = "Insulated In-Line Tank Small Liquid",
            desc = "",
            value = "-813426145"
        )
    )]
    StructureInsulatedInLineTankLiquid1X1 = -813426145i32,
    #[strum(serialize = "StructureChuteDigitalFlipFlopSplitterLeft")]
    #[strum(
        props(
            name = "Chute Digital Flip Flop Splitter Left",
            desc = "The digital flip flop will toggle between two outputs using a specified ratio (n:1). For example, setting the dial to 2 would allow two items to pass through the primary output before flipping to the secondary output.",
            value = "-810874728"
        )
    )]
    StructureChuteDigitalFlipFlopSplitterLeft = -810874728i32,
    #[strum(serialize = "MotherboardRockets")]
    #[strum(props(name = "Rocket Control Motherboard", desc = "", value = "-806986392"))]
    MotherboardRockets = -806986392i32,
    #[strum(serialize = "ItemKitFurnace")]
    #[strum(props(name = "Kit (Furnace)", desc = "", value = "-806743925"))]
    ItemKitFurnace = -806743925i32,
    #[strum(serialize = "ItemTropicalPlant")]
    #[strum(
        props(
            name = "Tropical Lily",
            desc = "An anthurium, evolved in the jungles of South America, which will tolerate higher temperatures than most plants.",
            value = "-800947386"
        )
    )]
    ItemTropicalPlant = -800947386i32,
    #[strum(serialize = "ItemKitLiquidTank")]
    #[strum(props(name = "Kit (Liquid Tank)", desc = "", value = "-799849305"))]
    ItemKitLiquidTank = -799849305i32,
    #[strum(serialize = "StructureCompositeDoor")]
    #[strum(
        props(
            name = "Composite Door",
            desc = "<link=Recurso><color=#0080FFFF>Recurso's</color></link> composite doors are rated to 300kPa, which is more than sufficient for most purposes they were designed for. However, steep pressure differentials are not your friend.",
            value = "-793837322"
        )
    )]
    StructureCompositeDoor = -793837322i32,
    #[strum(serialize = "StructureStorageLocker")]
    #[strum(props(name = "Locker", desc = "", value = "-793623899"))]
    StructureStorageLocker = -793623899i32,
    #[strum(serialize = "RespawnPoint")]
    #[strum(
        props(
            name = "Respawn Point",
            desc = "Place a respawn point to set a player entry point to your base when loading in, or returning from the dead.",
            value = "-788672929"
        )
    )]
    RespawnPoint = -788672929i32,
    #[strum(serialize = "ItemInconelIngot")]
    #[strum(props(name = "Ingot (Inconel)", desc = "", value = "-787796599"))]
    ItemInconelIngot = -787796599i32,
    #[strum(serialize = "StructurePoweredVentLarge")]
    #[strum(
        props(
            name = "Powered Vent Large",
            desc = "For building large scale airlock systems and pressurised hangers, a bigger and bolder version of the <link=ThingStructurePoweredVent><color=green>Powered Vent</color></link> that can effeciently pull a vacuum in large room.",
            value = "-785498334"
        )
    )]
    StructurePoweredVentLarge = -785498334i32,
    #[strum(serialize = "ItemKitWallGeometry")]
    #[strum(props(name = "Kit (Geometric Wall)", desc = "", value = "-784733231"))]
    ItemKitWallGeometry = -784733231i32,
    #[strum(serialize = "StructureInsulatedPipeCrossJunction4")]
    #[strum(
        props(
            name = "Insulated Pipe (4-Way Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "-783387184"
        )
    )]
    StructureInsulatedPipeCrossJunction4 = -783387184i32,
    #[strum(serialize = "StructurePowerConnector")]
    #[strum(
        props(
            name = "Power Connector",
            desc = "Attaches a <link=ThingItemKitDynamicGenerator><color=green>Kit (Portable Generator)</color></link> to a power network.",
            value = "-782951720"
        )
    )]
    StructurePowerConnector = -782951720i32,
    #[strum(serialize = "StructureLiquidPipeOneWayValve")]
    #[strum(
        props(
            name = "One Way Valve (Liquid)",
            desc = "The one way valve moves liquid in one direction only: from input side to output side. It only permits flow if the input pressure is higher than output pressure..",
            value = "-782453061"
        )
    )]
    StructureLiquidPipeOneWayValve = -782453061i32,
    #[strum(serialize = "StructureWallLargePanelArrow")]
    #[strum(props(name = "Wall (Large Panel Arrow)", desc = "", value = "-776581573"))]
    StructureWallLargePanelArrow = -776581573i32,
    #[strum(serialize = "StructureShower")]
    #[strum(props(name = "Shower", desc = "", value = "-775128944"))]
    StructureShower = -775128944i32,
    #[strum(serialize = "ItemChemLightBlue")]
    #[strum(
        props(
            name = "Chem Light (Blue)",
            desc = "A safe and slightly rave-some source of blue light. Snap to activate.",
            value = "-772542081"
        )
    )]
    ItemChemLightBlue = -772542081i32,
    #[strum(serialize = "StructureLogicSlotReader")]
    #[strum(props(name = "Slot Reader", desc = "", value = "-767867194"))]
    StructureLogicSlotReader = -767867194i32,
    #[strum(serialize = "ItemGasCanisterCarbonDioxide")]
    #[strum(props(name = "Canister (CO2)", desc = "", value = "-767685874"))]
    ItemGasCanisterCarbonDioxide = -767685874i32,
    #[strum(serialize = "ItemPipeAnalyizer")]
    #[strum(
        props(
            name = "Kit (Pipe Analyzer)",
            desc = "This kit creates a <link=ThingStructurePipeAnalysizer><color=green>Pipe Analyzer</color></link>.",
            value = "-767597887"
        )
    )]
    ItemPipeAnalyizer = -767597887i32,
    #[strum(serialize = "StructureBatteryChargerSmall")]
    #[strum(props(name = "Battery Charger Small", desc = "", value = "-761772413"))]
    StructureBatteryChargerSmall = -761772413i32,
    #[strum(serialize = "StructureWaterBottleFillerPowered")]
    #[strum(props(name = "Waterbottle Filler", desc = "", value = "-756587791"))]
    StructureWaterBottleFillerPowered = -756587791i32,
    #[strum(serialize = "AppliancePackagingMachine")]
    #[strum(
        props(
            name = "Basic Packaging Machine",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> Cannifier requires <link=ThingItemEmptyCan><color=green>Empty Can</color></link> and cooked <link=OrganicPage><color=#0080FFFF>food</color></link> to create <link=DecayPage><color=#0080FFFF>long-lasting</color></link>, easily stored sustenance. Note that the Cannifier must be bolted to a <link=ThingStructureBench><color=green>Powered Bench</color></link> for power, and only accepts cooked food and tin cans.\n\n<size=120%><b>OPERATION</b></size>\n\n1. Add the correct ingredients to the device via the hopper in the TOP.\n\n2. Close the device using the dropdown handle.\n\n3. Activate the device.\n\n4. Remove canned goods from the outlet in the FRONT.\n\nNote: the Cannifier will flash an error on its activation switch if you attempt to activate it before closing it.\n\n\n      ",
            value = "-749191906"
        )
    )]
    AppliancePackagingMachine = -749191906i32,
    #[strum(serialize = "ItemIntegratedCircuit10")]
    #[strum(props(name = "Integrated Circuit (IC10)", desc = "", value = "-744098481"))]
    ItemIntegratedCircuit10 = -744098481i32,
    #[strum(serialize = "ItemLabeller")]
    #[strum(
        props(
            name = "Labeller",
            desc = "A labeller lets you set names and values on a variety of devices and structures, including <link=ThingStructureConsole><color=green>Console</color></link> and <link=LogicPage><color=#0080FFFF>Logic</color></link>.",
            value = "-743968726"
        )
    )]
    ItemLabeller = -743968726i32,
    #[strum(serialize = "StructureCableJunctionH4")]
    #[strum(
        props(name = "Heavy Cable (4-Way Junction)", desc = "", value = "-742234680")
    )]
    StructureCableJunctionH4 = -742234680i32,
    #[strum(serialize = "StructureWallCooler")]
    #[strum(
        props(
            name = "Wall Cooler",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> Freezy Boi wall cooler complements the wall heater, which can only raise the temperature. The wall cooler functions by drawing heat from the surrounding atmosphere and adding that heat into its pipe network.\nIn order to run the wall cooler properly, you will need to connect pipes to the wall cooler and fill the connected pipe network with any type of gas. The gas's heat capacity and volume will determine how fast it reacts to temperature changes.\n\n<size=120%><b>EFFICIENCY</b></size>\nThe higher the difference in temperature between the gas stored in the pipes and the room, the less efficient the wall cooler will be. So to keep the wall cooler running at an acceptable efficiency you will need to get rid of the heat that accumulates in the pipes connected to it. A common practice would be to run the pipes to the outside and use radiators on the outside section of the pipes to get rid of the heat.\nThe less efficient the wall cooler, the less power it consumes. It will consume 1010W at max efficiency. The wall cooler can be controlled by logic chips to run when the temperature hits a certain degree.\n<size=120%><b>ERRORS</b></size>\nIf the wall cooler is flashing an error then it is missing one of the following:\n\n- Pipe connection to the wall cooler.\n- Gas in the connected pipes, or pressure is too low.\n- Atmosphere in the surrounding environment or pressure is too low.\n\nFor more information about how to control temperatures, consult the <link=TemperatureControlPage><color=#0080FFFF>temperature control</color></link> Guides page.",
            value = "-739292323"
        )
    )]
    StructureWallCooler = -739292323i32,
    #[strum(serialize = "StructurePurgeValve")]
    #[strum(
        props(
            name = "Purge Valve",
            desc = "Allows for removal of pressurant gas and evaporated liquids from a liquid pipe. Similar in function to a <link=ThingStructureBackPressureRegulator><color=green>Back Pressure Regulator</color></link> the <link=ThingStructurePurgeValve><color=green>Purge Valve</color></link> moves gas from the input liquid pipe to the output gas pipe aiming to keep the pressure of the input at the target setting.",
            value = "-737232128"
        )
    )]
    StructurePurgeValve = -737232128i32,
    #[strum(serialize = "StructureCrateMount")]
    #[strum(props(name = "Container Mount", desc = "", value = "-733500083"))]
    StructureCrateMount = -733500083i32,
    #[strum(serialize = "ItemKitDynamicGenerator")]
    #[strum(props(name = "Kit (Portable Generator)", desc = "", value = "-732720413"))]
    ItemKitDynamicGenerator = -732720413i32,
    #[strum(serialize = "StructureConsoleDual")]
    #[strum(
        props(
            name = "Console Dual",
            desc = "This <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> control box manages devices such as the <link=ThingStructureActiveVent><color=green>Active Vent</color></link>, <link=ThingStructureGasSensor><color=green>Gas Sensor</color></link>, <link=ThingStructureCompositeDoor><color=green>Composite Door</color></link> and others, depending on which <link=LogicPage><color=#0080FFFF>circuitboard</color></link> is inserted into the unit. It has separate data and power ports.\nA completed console displays all devices connected to the current power network. Any devices not related to the installed circuitboard will be greyed-out and inoperable. Consoles are locked once a <link=ThingItemDataDisk><color=green>Data Disk</color></link> is removed.",
            value = "-722284333"
        )
    )]
    StructureConsoleDual = -722284333i32,
    #[strum(serialize = "ItemGasFilterOxygen")]
    #[strum(
        props(
            name = "Filter (Oxygen)",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai</color></link> have cornered the market in filter design. Their trademarked templates are simple to print and highly efficient at capturing various gases, which can be disposed of or used elsewhere. <link=GasOxygen><color=#44AD83>Oxygen</color></link> is a common byproduct of smelting various ores, but must be filtered of such impurities as <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> using this filter and various devices, such as the <link=ThingItemDynamicScrubber><color=green>Kit (Portable Scrubber)</color></link>.",
            value = "-721824748"
        )
    )]
    ItemGasFilterOxygen = -721824748i32,
    #[strum(serialize = "ItemCookedTomato")]
    #[strum(
        props(
            name = "Cooked Tomato",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "-709086714"
        )
    )]
    ItemCookedTomato = -709086714i32,
    #[strum(serialize = "ItemCopperOre")]
    #[strum(
        props(
            name = "Ore (Copper)",
            desc = "Copper is a chemical element with the symbol \"Cu\". This common and highly conductive material is found on most astronomical bodies and is used in a variety of manufacturing processes including electronic components, alloys, and wires.",
            value = "-707307845"
        )
    )]
    ItemCopperOre = -707307845i32,
    #[strum(serialize = "StructureLogicTransmitter")]
    #[strum(
        props(
            name = "Logic Transmitter",
            desc = "Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "-693235651"
        )
    )]
    StructureLogicTransmitter = -693235651i32,
    #[strum(serialize = "StructureValve")]
    #[strum(props(name = "Valve", desc = "", value = "-692036078"))]
    StructureValve = -692036078i32,
    #[strum(serialize = "StructureCompositeWindowIron")]
    #[strum(props(name = "Iron Window", desc = "", value = "-688284639"))]
    StructureCompositeWindowIron = -688284639i32,
    #[strum(serialize = "ItemSprayCanBlack")]
    #[strum(
        props(
            name = "Spray Paint (Black)",
            desc = "Go classic, clandestine or just plain Gothic with black paint, which can be applied to most items. Each can has 20 uses.",
            value = "-688107795"
        )
    )]
    ItemSprayCanBlack = -688107795i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadLongTerm")]
    #[strum(
        props(name = "Mining-Drill Head (Long Term)", desc = "", value = "-684020753")
    )]
    ItemRocketMiningDrillHeadLongTerm = -684020753i32,
    #[strum(serialize = "ItemMiningBelt")]
    #[strum(
        props(
            name = "Mining Belt",
            desc = "Originally developed by <link=Recurso><color=#0080FFFF>Recurso Espaciais</color></link> for asteroid mining, the <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> mining belt has room for two <link=ToolPage><color=#0080FFFF>tools</color></link> and eight <link=OrePage><color=#0080FFFF>ore</color></link> stacks. While wearing the belt, <link=OrePage><color=#0080FFFF>ore</color></link> is automatically stored there when mined. Volatile and temperature-dependent remain stable in the environmentally controlled unit.",
            value = "-676435305"
        )
    )]
    ItemMiningBelt = -676435305i32,
    #[strum(serialize = "ItemGasCanisterSmart")]
    #[strum(
        props(
            name = "Gas Canister (Smart)",
            desc = "0.Mode0\n1.Mode1",
            value = "-668314371"
        )
    )]
    ItemGasCanisterSmart = -668314371i32,
    #[strum(serialize = "ItemFlour")]
    #[strum(
        props(
            name = "Flour",
            desc = "Pulverized <link=ThingItemWheat><color=green>Wheat</color></link>, a key ingredient in many foods created by the <link=ThingApplianceMicrowave><color=green>Microwave</color></link> and the <link=ThingItemKitAutomatedOven><color=green>Kit (Automated Oven)</color></link>.",
            value = "-665995854"
        )
    )]
    ItemFlour = -665995854i32,
    #[strum(serialize = "StructureSmallTableRectangleDouble")]
    #[strum(
        props(name = "Small (Table Rectangle Double)", desc = "", value = "-660451023")
    )]
    StructureSmallTableRectangleDouble = -660451023i32,
    #[strum(serialize = "StructureChuteUmbilicalFemaleSide")]
    #[strum(
        props(name = "Umbilical Socket Angle (Chute)", desc = "", value = "-659093969")
    )]
    StructureChuteUmbilicalFemaleSide = -659093969i32,
    #[strum(serialize = "ItemSteelIngot")]
    #[strum(
        props(
            name = "Ingot (Steel)",
            desc = "Steel ingots are a metal alloy, crafted in a <link=ThingStructureFurnace><color=green>Furnace</color></link> by smelting <link=ThingItemIronOre><color=green>Ore (Iron)</color></link> and <link=ThingItemCoalOre><color=green>Ore (Coal)</color></link> at a ratio of 3:1.\nIt may not be elegant, but <link=ThingItemOxite><color=green>Ice (Oxite)</color></link> and <link=ThingItemVolatiles><color=green>Ice (Volatiles)</color></link> can be combined at a ratio of 1:2 in a furnace to create the necessary gas mixture for smelting.",
            value = "-654790771"
        )
    )]
    ItemSteelIngot = -654790771i32,
    #[strum(serialize = "SeedBag_Wheet")]
    #[strum(
        props(
            name = "Wheat Seeds",
            desc = "Grow some <link=ThingItemWheat><color=green>Wheat</color></link>.",
            value = "-654756733"
        )
    )]
    SeedBagWheet = -654756733i32,
    #[strum(serialize = "StructureRocketTower")]
    #[strum(props(name = "Launch Tower", desc = "", value = "-654619479"))]
    StructureRocketTower = -654619479i32,
    #[strum(serialize = "StructureGasUmbilicalFemaleSide")]
    #[strum(
        props(name = "Umbilical Socket Angle (Gas)", desc = "", value = "-648683847")
    )]
    StructureGasUmbilicalFemaleSide = -648683847i32,
    #[strum(serialize = "StructureLockerSmall")]
    #[strum(props(name = "Locker (Small)", desc = "", value = "-647164662"))]
    StructureLockerSmall = -647164662i32,
    #[strum(serialize = "StructureSecurityPrinter")]
    #[strum(
        props(
            name = "Security Printer",
            desc = "Any <link=Stationeer><color=#0080FFFF>Stationeer</color></link> concerned about security needs the <link=Harkwell><color=#0080FFFF>Harkwell-designed</color></link> Vigilant-E security printer. Use the Vigilant-E to create a <link=ThingCartridgeAccessController><color=green>Cartridge (Access Controller)</color></link>, in order to restrict access to different parts of your base via keycards like the <link=ThingAccessCardBlue><color=green>Access Card (Blue)</color></link>. The printer also makes a variety of weapons and ammunitions to defend your base against any hostile, aggressive or just slightly rude entites you encounter as you explore the Solar System.\n",
            value = "-641491515"
        )
    )]
    StructureSecurityPrinter = -641491515i32,
    #[strum(serialize = "StructureWallSmallPanelsArrow")]
    #[strum(props(name = "Wall (Small Panels Arrow)", desc = "", value = "-639306697"))]
    StructureWallSmallPanelsArrow = -639306697i32,
    #[strum(serialize = "ItemKitDynamicMKIILiquidCanister")]
    #[strum(
        props(name = "Kit (Portable Liquid Tank Mk II)", desc = "", value = "-638019974")
    )]
    ItemKitDynamicMkiiLiquidCanister = -638019974i32,
    #[strum(serialize = "ItemKitRocketManufactory")]
    #[strum(props(name = "Kit (Rocket Manufactory)", desc = "", value = "-636127860"))]
    ItemKitRocketManufactory = -636127860i32,
    #[strum(serialize = "ItemPureIceVolatiles")]
    #[strum(
        props(
            name = "Pure Ice Volatiles",
            desc = "A frozen chunk of pure <link=GasVolatiles><color=#44AD83>Volatiles</color></link>",
            value = "-633723719"
        )
    )]
    ItemPureIceVolatiles = -633723719i32,
    #[strum(serialize = "ItemGasFilterNitrogenM")]
    #[strum(props(name = "Medium Filter (Nitrogen)", desc = "", value = "-632657357"))]
    ItemGasFilterNitrogenM = -632657357i32,
    #[strum(serialize = "StructureCableFuse5k")]
    #[strum(props(name = "Fuse (5kW)", desc = "", value = "-631590668"))]
    StructureCableFuse5K = -631590668i32,
    #[strum(serialize = "StructureCableJunction6Burnt")]
    #[strum(
        props(name = "Burnt Cable (6-Way Junction)", desc = "", value = "-628145954")
    )]
    StructureCableJunction6Burnt = -628145954i32,
    #[strum(serialize = "StructureComputer")]
    #[strum(
        props(
            name = "Computer",
            desc = "In some ways a relic, the 'Chonk R1' was designed by severely conflicted <link=Norsec><color=#0080FFFF>Norsec</color></link> technicians, who needed a unit that could operate with a wide range of <link=LogicUnitPage><color=#0080FFFF>motherboards</color></link>, while also enduring the worst a new Cadet could throw at it.\nThe result is a machine described by some as 'the only PC likely to survive our collision with a black hole', while other, less appreciative users regard it as sharing most of its technological DNA with a cheese grater.\nCompatible motherboards:\n- <link=ThingMotherboardLogic><color=green>Logic Motherboard</color></link>\n- <link=ThingMotherboardManufacturing><color=green>Manufacturing Motherboard</color></link>\n- <link=ThingMotherboardSorter><color=green>Sorter Motherboard</color></link>\n- <link=ThingMotherboardComms><color=green>Communications Motherboard</color></link>\n- <link=ThingMotherboardProgrammableChip><color=green>IC Editor Motherboard</color></link>",
            value = "-626563514"
        )
    )]
    StructureComputer = -626563514i32,
    #[strum(serialize = "StructurePressureFedGasEngine")]
    #[strum(
        props(
            name = "Pressure Fed Gas Engine",
            desc = "Inefficient but very powerful, the Pressure Fed Gas Engine moves gas from each of its two inputs based on the pressure of the input pipes. Control the mixing ratio of fuels by tweaking the input pressures to target a 2:1 mix of <link=GasVolatiles><color=#44AD83>Volatiles</color></link> to <link=GasOxygen><color=#44AD83>Oxygen</color></link> gas. Chilling propellant gasses or using <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> as an oxydizer will result in even higher thrust outputs.",
            value = "-624011170"
        )
    )]
    StructurePressureFedGasEngine = -624011170i32,
    #[strum(serialize = "StructureGroundBasedTelescope")]
    #[strum(
        props(
            name = "Telescope",
            desc = "A telescope that can be oriented to observe Celestial Bodies. When within full alignment will show orbital information for that celestial object. Atmospheric conditions may disrupt the ability to observe some objects at some times of day. To collect Horizontal and Vertical values you can use a <link=ThingStructureRocketCelestialTracker><color=green>Rocket Celestial Tracker</color></link> while it is in orbit, or a <link=ThingStructureDaylightSensor><color=green>Daylight Sensor</color></link> for primary body data.",
            value = "-619745681"
        )
    )]
    StructureGroundBasedTelescope = -619745681i32,
    #[strum(serialize = "ItemKitAdvancedFurnace")]
    #[strum(props(name = "Kit (Advanced Furnace)", desc = "", value = "-616758353"))]
    ItemKitAdvancedFurnace = -616758353i32,
    #[strum(serialize = "StructureHeatExchangerLiquidtoLiquid")]
    #[strum(
        props(
            name = "Heat Exchanger - Liquid",
            desc = "The original specs for the N Series Flow-P heat exchanger were rumored to have been scrawled on the back of a burger receipt by a bored <link=Sinotai><color=#0080FFFF>Sinotai</color></link> designer riding up the Brazilian space elevator, but that hasn't stopped it becoming one of the most widely-copied heat exchanger designs in the Solar System.\nThe 'N Flow-P' has four connections, allowing you to pass two liquid networks into the unit, which then works to equalize temperature across the two separate networks.\nAs the N Flow-P is a passive system, it equalizes pressure across the entire of each individual network, unless connected to liquid management devices like a <link=ThingStructureLiquidVolumePump><color=green>Liquid Volume Pump</color></link> or a <link=ThingStructureBackLiquidPressureRegulator><color=green>Liquid Back Volume Regulator</color></link>.\n",
            value = "-613784254"
        )
    )]
    StructureHeatExchangerLiquidtoLiquid = -613784254i32,
    #[strum(serialize = "StructureChuteJunction")]
    #[strum(
        props(
            name = "Chute (Junction)",
            desc = "The aim for any <link=Stationeers><color=#0080FFFF>Stationeer</color></link> is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nChute junctions are fundamental components of chute networks, allowing merging or splitting of these networks. When combined with a programmed <link=ThingStructureSorter><color=green>Sorter</color></link>, items can be sent down different paths to various machines with <link=ImportExportPage><color=#0080FFFF>import/export</color></link> slots.",
            value = "-611232514"
        )
    )]
    StructureChuteJunction = -611232514i32,
    #[strum(serialize = "StructureChuteWindow")]
    #[strum(
        props(
            name = "Chute (Window)",
            desc = "Chute's with windows let you see what's passing through your <link=ImportExportPage><color=#0080FFFF>import/export</color></link> network. But be warned, they are not insulated as other chutes are. Ices will melt.",
            value = "-607241919"
        )
    )]
    StructureChuteWindow = -607241919i32,
    #[strum(serialize = "ItemWearLamp")]
    #[strum(props(name = "Headlamp", desc = "", value = "-598730959"))]
    ItemWearLamp = -598730959i32,
    #[strum(serialize = "ItemKitAdvancedPackagingMachine")]
    #[strum(
        props(name = "Kit (Advanced Packaging Machine)", desc = "", value = "-598545233")
    )]
    ItemKitAdvancedPackagingMachine = -598545233i32,
    #[strum(serialize = "ItemChemLightGreen")]
    #[strum(
        props(
            name = "Chem Light (Green)",
            desc = "Enliven the dreariest, airless rock with this glowy green light. Snap to activate.",
            value = "-597479390"
        )
    )]
    ItemChemLightGreen = -597479390i32,
    #[strum(serialize = "EntityRoosterBrown")]
    #[strum(
        props(
            name = "Entity Rooster Brown",
            desc = "The common brown rooster. Don't let it hear you say that.",
            value = "-583103395"
        )
    )]
    EntityRoosterBrown = -583103395i32,
    #[strum(serialize = "StructureLargeExtendableRadiator")]
    #[strum(
        props(
            name = "Large Extendable Radiator",
            desc = "Omptimised for radiating heat in vacuum and low pressure environments. If pointed at the sun it will heat its contents rapidly via solar heating. The panels can fold away to stop all heat radiation/solar heating and protect them from storms.",
            value = "-566775170"
        )
    )]
    StructureLargeExtendableRadiator = -566775170i32,
    #[strum(serialize = "StructureMediumHangerDoor")]
    #[strum(
        props(
            name = "Medium Hangar Door",
            desc = "1 x 2 modular door piece for building hangar doors.",
            value = "-566348148"
        )
    )]
    StructureMediumHangerDoor = -566348148i32,
    #[strum(serialize = "StructureLaunchMount")]
    #[strum(
        props(
            name = "Launch Mount",
            desc = "The first piece to place whern building a rocket. Rockets can be constructed and/or landed here. Each Launch Mount will be allocated a slot on the Space Map and assigned a Location Code.",
            value = "-558953231"
        )
    )]
    StructureLaunchMount = -558953231i32,
    #[strum(serialize = "StructureShortLocker")]
    #[strum(props(name = "Short Locker", desc = "", value = "-554553467"))]
    StructureShortLocker = -554553467i32,
    #[strum(serialize = "ItemKitCrateMount")]
    #[strum(props(name = "Kit (Container Mount)", desc = "", value = "-551612946"))]
    ItemKitCrateMount = -551612946i32,
    #[strum(serialize = "ItemKitCryoTube")]
    #[strum(props(name = "Kit (Cryo Tube)", desc = "", value = "-545234195"))]
    ItemKitCryoTube = -545234195i32,
    #[strum(serialize = "StructureSolarPanelDual")]
    #[strum(
        props(
            name = "Solar Panel (Dual)",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai</color></link> dual solar panels are used for generating power from sunlight, with dedicated data and power ports. They can be connected to {<link=LogicPage><color=#0080FFFF>Logic</color></link> systems, in order to track sunlight, but their efficiency is reduced during storms and when damaged. You can repair these using some trusty <link=ThingItemDuctTape><color=green>Duct Tape</color></link>.",
            value = "-539224550"
        )
    )]
    StructureSolarPanelDual = -539224550i32,
    #[strum(serialize = "ItemPlantSwitchGrass")]
    #[strum(props(name = "Switch Grass", desc = "", value = "-532672323"))]
    ItemPlantSwitchGrass = -532672323i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidTJunction")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (T Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "-532384855"
        )
    )]
    StructureInsulatedPipeLiquidTJunction = -532384855i32,
    #[strum(serialize = "ItemKitSolarPanelBasicReinforced")]
    #[strum(
        props(name = "Kit (Solar Panel Basic Heavy)", desc = "", value = "-528695432")
    )]
    ItemKitSolarPanelBasicReinforced = -528695432i32,
    #[strum(serialize = "ItemChemLightRed")]
    #[strum(
        props(
            name = "Chem Light (Red)",
            desc = "A red glowstick. Snap to activate. Then reach for the lasers.",
            value = "-525810132"
        )
    )]
    ItemChemLightRed = -525810132i32,
    #[strum(serialize = "ItemKitWallIron")]
    #[strum(props(name = "Kit (Iron Wall)", desc = "", value = "-524546923"))]
    ItemKitWallIron = -524546923i32,
    #[strum(serialize = "ItemEggCarton")]
    #[strum(
        props(
            name = "Egg Carton",
            desc = "Within, eggs reside in mysterious, marmoreal silence.",
            value = "-524289310"
        )
    )]
    ItemEggCarton = -524289310i32,
    #[strum(serialize = "StructureWaterDigitalValve")]
    #[strum(props(name = "Liquid Digital Valve", desc = "", value = "-517628750"))]
    StructureWaterDigitalValve = -517628750i32,
    #[strum(serialize = "StructureSmallDirectHeatExchangeLiquidtoLiquid")]
    #[strum(
        props(
            name = "Small Direct Heat Exchanger - Liquid + Liquid",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "-507770416"
        )
    )]
    StructureSmallDirectHeatExchangeLiquidtoLiquid = -507770416i32,
    #[strum(serialize = "ItemWirelessBatteryCellExtraLarge")]
    #[strum(
        props(
            name = "Wireless Battery Cell Extra Large",
            desc = "0.Empty\n1.Critical\n2.VeryLow\n3.Low\n4.Medium\n5.High\n6.Full",
            value = "-504717121"
        )
    )]
    ItemWirelessBatteryCellExtraLarge = -504717121i32,
    #[strum(serialize = "ItemGasFilterPollutantsInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Pollutants)",
            desc = "A filter that selectively targets Pollutants. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-503738105"
        )
    )]
    ItemGasFilterPollutantsInfinite = -503738105i32,
    #[strum(serialize = "ItemSprayCanBlue")]
    #[strum(
        props(
            name = "Spray Paint (Blue)",
            desc = "What kind of a color is blue? The kind of of color that says, 'Hey, what about me?'",
            value = "-498464883"
        )
    )]
    ItemSprayCanBlue = -498464883i32,
    #[strum(serialize = "RespawnPointWallMounted")]
    #[strum(props(name = "Respawn Point (Mounted)", desc = "", value = "-491247370"))]
    RespawnPointWallMounted = -491247370i32,
    #[strum(serialize = "ItemIronSheets")]
    #[strum(props(name = "Iron Sheets", desc = "", value = "-487378546"))]
    ItemIronSheets = -487378546i32,
    #[strum(serialize = "ItemGasCanisterVolatiles")]
    #[strum(props(name = "Canister (Volatiles)", desc = "", value = "-472094806"))]
    ItemGasCanisterVolatiles = -472094806i32,
    #[strum(serialize = "ItemCableCoil")]
    #[strum(
        props(
            name = "Cable Coil",
            desc = "Bodily metaphors are tired and anthropocentric, but it was Frida Stuppen, the first <link=ODA><color=#0080FFFF>ODA</color></link> Administrator, who said, 'Let the cabling be as the nerve and the vessel, transmitting power and data alike through systems we forge among the stars.' Later commentators suggested that she was simply putting a romantic gloss on a piece of dubious economy. Whatever the case, standard cabling is where any <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> network begins. \nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "-466050668"
        )
    )]
    ItemCableCoil = -466050668i32,
    #[strum(serialize = "StructureToolManufactory")]
    #[strum(
        props(
            name = "Tool Manufactory",
            desc = "No mission can be completed without the proper tools. The <link=Norsec><color=#0080FFFF>Norsec</color></link> ThuulDek manufactory can fabricate almost any tool or hand-held device a <link=Stationeer><color=#0080FFFF>Stationeer</color></link> may need to complete their mission, as well as a variety of delightful paints.\nUpgrade the device using a <link=ThingToolPrinterMod><color=green>Tool Printer Mod</color></link> for additional recipes and faster processing speeds.",
            value = "-465741100"
        )
    )]
    StructureToolManufactory = -465741100i32,
    #[strum(serialize = "StructureAdvancedPackagingMachine")]
    #[strum(
        props(
            name = "Advanced Packaging Machine",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> Advanced Cannifier Multi-Plus Pro is an automateable packaging machine that uses <link=ThingItemEmptyCan><color=green>Empty Can</color></link>s and cooked <link=OrganicPage><color=#0080FFFF>food</color></link> to create canned sustenance that does not decay. Note that the ACMPP only accepts cooked food and tin cans.",
            value = "-463037670"
        )
    )]
    StructureAdvancedPackagingMachine = -463037670i32,
    #[strum(serialize = "Battery_Wireless_cell")]
    #[strum(
        props(
            name = "Battery Wireless Cell",
            desc = "0.Empty\n1.Critical\n2.VeryLow\n3.Low\n4.Medium\n5.High\n6.Full",
            value = "-462415758"
        )
    )]
    BatteryWirelessCell = -462415758i32,
    #[strum(serialize = "ItemBatteryCellLarge")]
    #[strum(
        props(
            name = "Battery Cell (Large)",
            desc = "First mass-produced by <link=Xigo><color=#0080FFFF>Xigo</color></link> in 2155 on the basis of a unattributed prototype, the classic silicon anode solid-state design extends its optimum temperature range.\n\n<size=120%><b>POWER OUTPUT</b></size>\nThe large power cell can discharge 288kW of power. \n",
            value = "-459827268"
        )
    )]
    ItemBatteryCellLarge = -459827268i32,
    #[strum(serialize = "StructureLiquidVolumePump")]
    #[strum(props(name = "Liquid Volume Pump", desc = "", value = "-454028979"))]
    StructureLiquidVolumePump = -454028979i32,
    #[strum(serialize = "ItemKitTransformer")]
    #[strum(props(name = "Kit (Transformer Large)", desc = "", value = "-453039435"))]
    ItemKitTransformer = -453039435i32,
    #[strum(serialize = "StructureVendingMachine")]
    #[strum(
        props(
            name = "Vending Machine",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo-designed</color></link> 'Slot Mate' vending machine allows storage of almost any item, while also operating as a distribution point for working with <link=TradingPage><color=#0080FFFF>Traders</color></link>. You cannot trade without a vending machine, or its more advanced equivalent, the <link=ThingStructureRefrigeratedVendingMachine><color=green>Refrigerated Vending Machine</color></link>. Each vending machine can hold up to 100 stacks.",
            value = "-443130773"
        )
    )]
    StructureVendingMachine = -443130773i32,
    #[strum(serialize = "StructurePipeHeater")]
    #[strum(
        props(
            name = "Pipe Heater (Gas)",
            desc = "Adds 1000 joules of heat per tick to the contents of your pipe network.",
            value = "-419758574"
        )
    )]
    StructurePipeHeater = -419758574i32,
    #[strum(serialize = "StructurePipeCrossJunction4")]
    #[strum(
        props(
            name = "Pipe (4-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCrossJunction4><color=green>Insulated Pipe (4-Way Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-417629293"
        )
    )]
    StructurePipeCrossJunction4 = -417629293i32,
    #[strum(serialize = "StructureLadder")]
    #[strum(props(name = "Ladder", desc = "", value = "-415420281"))]
    StructureLadder = -415420281i32,
    #[strum(serialize = "ItemHardJetpack")]
    #[strum(
        props(
            name = "Hardsuit Jetpack",
            desc = "The <link=Norsec><color=#0080FFFF>Norsec</color></link> jetpack isn't 'technically' a jetpack at all, it's a gas thruster. It can be powered by any gas, so long as the internal pressure of the <link=AtmosphericsPage><color=#0080FFFF>canister</color></link> is higher than the ambient external pressure. If the external pressure is greater, the spacepack will not function. Adjusting the thrust value alters your rate of acceleration, while activating the stablizer causes the spacepack to hover when a given height is reached.\nThe hardsuit jetpack is capable of much higher speeds than the <link=ThingItemJetpackBasic><color=green>Jetpack Basic</color></link> - up to 15m/s. Indispensable for building, mining and general movement, it has fourteen storage slots.\nUSE: 'J' to activate; 'space' to fly up; 'left ctrl' to descend; and 'WASD' to move.",
            value = "-412551656"
        )
    )]
    ItemHardJetpack = -412551656i32,
    #[strum(serialize = "CircuitboardCameraDisplay")]
    #[strum(
        props(
            name = "Camera Display",
            desc = "Surveillance is sometimes necessary when building bases in highly hostile environments. The camera display circuit board allows wary Stationeers to turn a <link=ThingStructureConsole><color=green>Console</color></link> into a security display when connected to a <link=ThingStructureCamera><color=green>Camera</color></link>.",
            value = "-412104504"
        )
    )]
    CircuitboardCameraDisplay = -412104504i32,
    #[strum(serialize = "ItemCopperIngot")]
    #[strum(
        props(
            name = "Ingot (Copper)",
            desc = "Copper ingots are created by smelting <link=ThingItemCopperOre><color=green>Ore (Copper)</color></link> in the <link=ThingStructureFurnace><color=green>Furnace</color></link> and <link=ThingStructureArcFurnace><color=green>Arc Furnace</color></link>, and used to create a variety of items.",
            value = "-404336834"
        )
    )]
    ItemCopperIngot = -404336834i32,
    #[strum(serialize = "ReagentColorOrange")]
    #[strum(props(name = "Color Dye (Orange)", desc = "", value = "-400696159"))]
    ReagentColorOrange = -400696159i32,
    #[strum(serialize = "StructureBattery")]
    #[strum(
        props(
            name = "Station Battery",
            desc = "Providing large-scale, reliable power storage, the <link=Sinotai><color=#0080FFFF>Sinotai</color></link> 'Dianzi' station battery is the heart of most <link=Stationeers><color=#0080FFFF>Stationeer</color></link> bases. \nThere are a variety of cautions to the design of electrical systems using batteries, and every experienced Stationeer has a story to tell, hence the Stationeer adage: 'Dianzi cooks, but it also frys.' \n<size=120%><b>POWER OUTPUT</b></size>\nAble to store up to 3600000W of power, there are no practical limits to its throughput, hence it is wise to use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>. Seasoned electrical engineers will also laugh in the face of those who fail to separate out their power generation networks using an <link=ThingStructureAreaPowerControl><color=green>Area Power Control</color></link> and <link=ThingStructureTransformer><color=green>Transformer (Large)</color></link>.",
            value = "-400115994"
        )
    )]
    StructureBattery = -400115994i32,
    #[strum(serialize = "StructurePipeRadiatorFlat")]
    #[strum(
        props(
            name = "Pipe Radiator",
            desc = "A pipe mounted radiator optimized for radiating heat in vacuums.",
            value = "-399883995"
        )
    )]
    StructurePipeRadiatorFlat = -399883995i32,
    #[strum(serialize = "StructureCompositeCladdingAngledLong")]
    #[strum(
        props(name = "Composite Cladding (Long Angled)", desc = "", value = "-387546514")
    )]
    StructureCompositeCladdingAngledLong = -387546514i32,
    #[strum(serialize = "DynamicGasTankAdvanced")]
    #[strum(
        props(name = "Gas Tank Mk II", desc = "0.Mode0\n1.Mode1", value = "-386375420")
    )]
    DynamicGasTankAdvanced = -386375420i32,
    #[strum(serialize = "WeaponPistolEnergy")]
    #[strum(
        props(name = "Energy Pistol", desc = "0.Stun\n1.Kill", value = "-385323479")
    )]
    WeaponPistolEnergy = -385323479i32,
    #[strum(serialize = "ItemFertilizedEgg")]
    #[strum(
        props(
            name = "Egg",
            desc = "To hatch it requires an incubation temperature of between 35 and 45 degrees Celsius and will hatch into a <link=ThingNpcChick><color=green>Chick</color></link>. If the egg is exposed to tepratures below 10 degrees it will no longer be viable.",
            value = "-383972371"
        )
    )]
    ItemFertilizedEgg = -383972371i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadIce")]
    #[strum(props(name = "Mining-Drill Head (Ice)", desc = "", value = "-380904592"))]
    ItemRocketMiningDrillHeadIce = -380904592i32,
    #[strum(serialize = "Flag_ODA_8m")]
    #[strum(props(name = "Flag (ODA 8m)", desc = "", value = "-375156130"))]
    FlagOda8M = -375156130i32,
    #[strum(serialize = "AccessCardGreen")]
    #[strum(props(name = "Access Card (Green)", desc = "", value = "-374567952"))]
    AccessCardGreen = -374567952i32,
    #[strum(serialize = "StructureChairBoothCornerLeft")]
    #[strum(props(name = "Chair (Booth Corner Left)", desc = "", value = "-367720198"))]
    StructureChairBoothCornerLeft = -367720198i32,
    #[strum(serialize = "ItemKitFuselage")]
    #[strum(props(name = "Kit (Fuselage)", desc = "", value = "-366262681"))]
    ItemKitFuselage = -366262681i32,
    #[strum(serialize = "ItemSolidFuel")]
    #[strum(props(name = "Solid Fuel (Hydrocarbon)", desc = "", value = "-365253871"))]
    ItemSolidFuel = -365253871i32,
    #[strum(serialize = "ItemKitSolarPanelReinforced")]
    #[strum(props(name = "Kit (Solar Panel Heavy)", desc = "", value = "-364868685"))]
    ItemKitSolarPanelReinforced = -364868685i32,
    #[strum(serialize = "ItemToolBelt")]
    #[strum(
        props(
            name = "Tool Belt",
            desc = "If there's one piece of equipment that embodies <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life above all else, it's the humble toolbelt (Editor's note: a recent <link=ODA><color=#0080FFFF>ODA</color></link> survey of iconic Stationeer equipment also rated the smoking, toxic ruins of an over-pressurized <link=ThingStructureFurnace><color=green>Furnace</color></link> lying amid the charred remains of your latest base very highly).\nDesigned to meet the most strict-ish ODA safety standards, the toolbelt's eight slots hold one thing: tools, and <link=ThingItemCableCoil><color=green>Cable Coil</color></link>. Not to be confused with the <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>.",
            value = "-355127880"
        )
    )]
    ItemToolBelt = -355127880i32,
    #[strum(serialize = "ItemEmergencyAngleGrinder")]
    #[strum(props(name = "Emergency Angle Grinder", desc = "", value = "-351438780"))]
    ItemEmergencyAngleGrinder = -351438780i32,
    #[strum(serialize = "StructureCableFuse50k")]
    #[strum(props(name = "Fuse (50kW)", desc = "", value = "-349716617"))]
    StructureCableFuse50K = -349716617i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerLongR")]
    #[strum(
        props(
            name = "Composite Cladding (Long Angled Mirrored Corner)",
            desc = "",
            value = "-348918222"
        )
    )]
    StructureCompositeCladdingAngledCornerLongR = -348918222i32,
    #[strum(serialize = "StructureFiltration")]
    #[strum(
        props(
            name = "Filtration",
            desc = "The Filtration Unit is based on a long-standing <link=ExMin><color=#0080FFFF>ExMin</color></link> system, itself based on older designs of uncertain provenance. It is available in the <link=ThingItemKitAtmospherics><color=green>Kit (Atmospherics)</color></link>.\nThe device has nonetheless proven indispensable for <link=Stationeers><color=#0080FFFF>Stationeer</color></link> atmospheric systems, as it can filter two <link=GasPage><color=#0080FFFF>gases</color></link> simultaneously from a single pipe network using a dual filter array. The unit has an input, and a filter output as well as an unfiltered outlet for any residual gases.\n",
            value = "-348054045"
        )
    )]
    StructureFiltration = -348054045i32,
    #[strum(serialize = "StructureLogicReader")]
    #[strum(props(name = "Logic Reader", desc = "", value = "-345383640"))]
    StructureLogicReader = -345383640i32,
    #[strum(serialize = "ItemKitMotherShipCore")]
    #[strum(props(name = "Kit (Mothership)", desc = "", value = "-344968335"))]
    ItemKitMotherShipCore = -344968335i32,
    #[strum(serialize = "StructureCamera")]
    #[strum(
        props(
            name = "Camera",
            desc = "Nothing says 'I care' like a security camera that's been linked a <link=ThingStructureMotionSensor><color=green>Motion Sensor</color></link> and a <link=ThingStructureConsole><color=green>Console</color></link> fitted with a <link=ThingCircuitboardCameraDisplay><color=green>Camera Display</color></link>.\nBe there, even when you're not.",
            value = "-342072665"
        )
    )]
    StructureCamera = -342072665i32,
    #[strum(serialize = "StructureCableJunctionHBurnt")]
    #[strum(props(name = "Burnt Cable (Junction)", desc = "", value = "-341365649"))]
    StructureCableJunctionHBurnt = -341365649i32,
    #[strum(serialize = "MotherboardComms")]
    #[strum(
        props(
            name = "Communications Motherboard",
            desc = "When placed in a <link=ThingStructureComputer><color=green>Computer</color></link> and connected to a <link=ThingLandingpad_DataConnectionPiece><color=green>Landingpad Data And Power</color></link>, a <link=ThingStructureSatelliteDish><color=green>Medium Satellite Dish</color></link>, and a <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link> allows Stationeers to trade with suppliers. Adjust the horizontal and vertical attributes of the <link=ThingStructureSatelliteDish><color=green>Medium Satellite Dish</color></link> either directly or through logic. You need a communications signal of 95% or above to establish reliable communications with a trader. A minimum of a 3x3 clear pad area with a <link=ThingLandingpad_CenterPiece01><color=green>Landingpad Center</color></link> at the center is required for a trader to land.",
            value = "-337075633"
        )
    )]
    MotherboardComms = -337075633i32,
    #[strum(serialize = "AccessCardOrange")]
    #[strum(props(name = "Access Card (Orange)", desc = "", value = "-332896929"))]
    AccessCardOrange = -332896929i32,
    #[strum(serialize = "StructurePowerTransmitterOmni")]
    #[strum(props(name = "Power Transmitter Omni", desc = "", value = "-327468845"))]
    StructurePowerTransmitterOmni = -327468845i32,
    #[strum(serialize = "StructureGlassDoor")]
    #[strum(
        props(name = "Glass Door", desc = "0.Operate\n1.Logic", value = "-324331872")
    )]
    StructureGlassDoor = -324331872i32,
    #[strum(serialize = "DynamicGasCanisterCarbonDioxide")]
    #[strum(
        props(
            name = "Portable Gas Tank (CO2)",
            desc = "Portable gas tanks do one thing: store gas. To refill the tank, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or ... boom. Once it's full, you can refill a <link=ThingItemGasCanisterCarbonDioxide><color=green>Canister (CO2)</color></link> by attaching it to the tank's striped section. Or you could vent the tank's variable flow rate valve into a room and create an atmosphere ... of sorts.",
            value = "-322413931"
        )
    )]
    DynamicGasCanisterCarbonDioxide = -322413931i32,
    #[strum(serialize = "StructureVolumePump")]
    #[strum(
        props(
            name = "Volume Pump",
            desc = "The volume pump pumps pumpable gases. It also separates out pipe networks into separate networks.",
            value = "-321403609"
        )
    )]
    StructureVolumePump = -321403609i32,
    #[strum(serialize = "DynamicMKIILiquidCanisterWater")]
    #[strum(
        props(
            name = "Portable Liquid Tank Mk II (Water)",
            desc = "An insulated version of the <link=ThingDynamicMKIILiquidCanisterWater><color=green>Portable Liquid Tank Mk II (Water)</color></link>, for storing liquids without them gaining or losing temperature.",
            value = "-319510386"
        )
    )]
    DynamicMkiiLiquidCanisterWater = -319510386i32,
    #[strum(serialize = "ItemKitRocketBattery")]
    #[strum(props(name = "Kit (Rocket Battery)", desc = "", value = "-314072139"))]
    ItemKitRocketBattery = -314072139i32,
    #[strum(serialize = "ElectronicPrinterMod")]
    #[strum(
        props(
            name = "Electronic Printer Mod",
            desc = "Apply to an <link=ThingStructureElectronicsPrinter><color=green>Electronics Printer</color></link> with a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> or <link=ThingItemArcWelder><color=green>Arc Welder</color></link> to upgrade for increased processing speed and more recipe options.",
            value = "-311170652"
        )
    )]
    ElectronicPrinterMod = -311170652i32,
    #[strum(serialize = "ItemWreckageHydroponicsTray1")]
    #[strum(props(name = "Wreckage Hydroponics Tray", desc = "", value = "-310178617"))]
    ItemWreckageHydroponicsTray1 = -310178617i32,
    #[strum(serialize = "ItemKitRocketCelestialTracker")]
    #[strum(
        props(name = "Kit (Rocket Celestial Tracker)", desc = "", value = "-303008602")
    )]
    ItemKitRocketCelestialTracker = -303008602i32,
    #[strum(serialize = "StructureFrameSide")]
    #[strum(
        props(
            name = "Steel Frame (Side)",
            desc = "More durable than the <link=ThingStructureFrameIron><color=green>Iron Frame</color></link>, steel frames also provide variations for more ornate constructions.",
            value = "-302420053"
        )
    )]
    StructureFrameSide = -302420053i32,
    #[strum(serialize = "ItemInvarIngot")]
    #[strum(props(name = "Ingot (Invar)", desc = "", value = "-297990285"))]
    ItemInvarIngot = -297990285i32,
    #[strum(serialize = "StructureSmallTableThickSingle")]
    #[strum(props(name = "Small Table (Thick Single)", desc = "", value = "-291862981"))]
    StructureSmallTableThickSingle = -291862981i32,
    #[strum(serialize = "ItemSiliconIngot")]
    #[strum(props(name = "Ingot (Silicon)", desc = "", value = "-290196476"))]
    ItemSiliconIngot = -290196476i32,
    #[strum(serialize = "StructureLiquidPipeHeater")]
    #[strum(
        props(
            name = "Pipe Heater (Liquid)",
            desc = "Adds 1000 joules of heat per tick to the contents of your pipe network.",
            value = "-287495560"
        )
    )]
    StructureLiquidPipeHeater = -287495560i32,
    #[strum(serialize = "ItemChocolateCake")]
    #[strum(props(name = "Chocolate Cake", desc = "", value = "-261575861"))]
    ItemChocolateCake = -261575861i32,
    #[strum(serialize = "StructureStirlingEngine")]
    #[strum(
        props(
            name = "Stirling Engine",
            desc = "Harnessing an ancient thermal exploit, the <link=Recurso><color=#0080FFFF>Recurso</color></link> 'Libra' Stirling Engine generates power via the expansion and contraction of a working gas to drive pistons operating an electrical generator.\n      \nWhen high pressure hot gas is supplied into the input pipe, this gas will heat the hot side of the unit, then pass into the output pipe. The cooler side uses the room's ambient atmosphere, which must be kept at a lower temperature and pressure in order to create a differential. Add a working gas by inserting a <link=SlotGasCanister><color=orange>Gas Canister</color></link>. The unit must be deactivated when adding or removing canisters, or the working gas may leak into the surrounding atmosphere.\n \nGases with a low molecular mass make the most efficient working gases. Increasing the moles of working gas can result in a greater potential power output. However, overpressuring the unit may have ... sub-optimal results.",
            value = "-260316435"
        )
    )]
    StructureStirlingEngine = -260316435i32,
    #[strum(serialize = "StructureCompositeCladdingRounded")]
    #[strum(
        props(name = "Composite Cladding (Rounded)", desc = "", value = "-259357734")
    )]
    StructureCompositeCladdingRounded = -259357734i32,
    #[strum(serialize = "SMGMagazine")]
    #[strum(props(name = "SMG Magazine", desc = "", value = "-256607540"))]
    SmgMagazine = -256607540i32,
    #[strum(serialize = "ItemLiquidPipeHeater")]
    #[strum(
        props(
            name = "Pipe Heater Kit (Liquid)",
            desc = "Creates a <link=ThingStructureLiquidPipeHeater><color=green>Pipe Heater (Liquid)</color></link>.",
            value = "-248475032"
        )
    )]
    ItemLiquidPipeHeater = -248475032i32,
    #[strum(serialize = "StructureArcFurnace")]
    #[strum(
        props(
            name = "Arc Furnace",
            desc = "The simplest smelting system available to the average <link=Stationeers><color=#0080FFFF>Stationeer</color></link>, <link=Recurso><color=#0080FFFF>Recurso's</color></link> arc furnace was forged itself in the depths of the Solar System to help explorational geologists determine the purity of potential asteroidal mining targets.\nCo-opted by the <link=ODA><color=#0080FFFF>ODA</color></link>, it now provides Stationeers with a way to produce pure ingots of various resources.\nThe smelting process also releases a range of by product <link=GasPage><color=#0080FFFF>gases</color></link>, principally <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>, <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link>, <link=GasVolatiles><color=#44AD83>Volatiles</color></link> and <link=GasOxygen><color=#44AD83>Oxygen</color></link> in differing ratios. These can be recaptured from the atmosphere by filtering, but also make the arc furnace a risk in closed environments. \nUnlike the more advanced <link=ThingStructureFurnace><color=green>Furnace</color></link>, the arc furnace cannot create <link=IngotPage><color=#0080FFFF>alloys</color></link>.",
            value = "-247344692"
        )
    )]
    StructureArcFurnace = -247344692i32,
    #[strum(serialize = "ItemTablet")]
    #[strum(
        props(
            name = "Handheld Tablet",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> handheld 'Padi' tablet is an all-purpose data platform, provided as standard issue to all <link=Stationeers><color=#0080FFFF>Stationeers</color></link>. A dynamic multi-tool that accepts a range of <link=CartridgePage><color=#0080FFFF>cartridges</color></link>, the Padi becomes an <link=ThingCartridgeAtmosAnalyser><color=green>Atmos Analyzer</color></link> or <link=ThingCartridgeTracker><color=green>Tracker</color></link>, <link=ThingCartridgeMedicalAnalyser><color=green>Medical Analyzer</color></link>, <link=ThingCartridgeOreScanner><color=green>Ore Scanner</color></link>, <link=ThingCartridgeElectronicReader><color=green>eReader</color></link>, and various other functions.",
            value = "-229808600"
        )
    )]
    ItemTablet = -229808600i32,
    #[strum(serialize = "StructureGovernedGasEngine")]
    #[strum(
        props(
            name = "Pumped Gas Engine",
            desc = "The most reliable of all the rocket engines, the Pumped Gas Engine runs on a 2:1 mix of <link=GasVolatiles><color=#44AD83>Volatiles</color></link> to <link=GasOxygen><color=#44AD83>Oxygen</color></link> gas.",
            value = "-214232602"
        )
    )]
    StructureGovernedGasEngine = -214232602i32,
    #[strum(serialize = "StructureStairs4x2RailR")]
    #[strum(props(name = "Stairs with Rail (Right)", desc = "", value = "-212902482"))]
    StructureStairs4X2RailR = -212902482i32,
    #[strum(serialize = "ItemLeadOre")]
    #[strum(
        props(
            name = "Ore (Lead)",
            desc = "Lead is a chemical element with the symbol \"Pb\". It is a dense, heavy metal with a low melting point. Lead is a used to make a variety of things such as <link=IngotPage><color=#0080FFFF>alloys</color></link> like <link=ThingItemSolderIngot><color=green>Ingot (Solder)</color></link> and munitions.",
            value = "-190236170"
        )
    )]
    ItemLeadOre = -190236170i32,
    #[strum(serialize = "StructureBeacon")]
    #[strum(props(name = "Beacon", desc = "", value = "-188177083"))]
    StructureBeacon = -188177083i32,
    #[strum(serialize = "ItemGasFilterCarbonDioxideInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Carbon Dioxide)",
            desc = "A filter that selectively targets Carbon Dioxide. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-185568964"
        )
    )]
    ItemGasFilterCarbonDioxideInfinite = -185568964i32,
    #[strum(serialize = "ItemLiquidCanisterEmpty")]
    #[strum(props(name = "Liquid Canister", desc = "", value = "-185207387"))]
    ItemLiquidCanisterEmpty = -185207387i32,
    #[strum(serialize = "ItemMKIIWireCutters")]
    #[strum(
        props(
            name = "Mk II Wire Cutters",
            desc = "Wirecutters allow you to deconstruct various <link=StructurePage><color=#0080FFFF>structures</color></link>, as well as cross-lay cables when held in your non-active hand, and defuse explosives as needed. Wirecutters are stored in the <link=ThingItemToolBelt><color=green>Tool Belt</color></link>, along with other essential <link=ToolPage><color=#0080FFFF>tools</color></link>.",
            value = "-178893251"
        )
    )]
    ItemMkiiWireCutters = -178893251i32,
    #[strum(serialize = "ItemPlantThermogenic_Genepool1")]
    #[strum(
        props(
            name = "Hades Flower (Alpha strain)",
            desc = "The <link=Agrizero><color=#0080FFFF>Agrizero's</color></link>-created Hades Flower is the result of as dubious experiment to combine the allure of tropical plants with the comfort and homeliness of a heat pump. The plant breathes a 1:3 mix of <link=GasVolatiles><color=#44AD83>Volatiles</color></link> and <link=GasOxygen><color=#44AD83>Oxygen</color></link>, and exhales heated <link=GasPollutant><color=#44AD83>Pollutant</color></link>.",
            value = "-177792789"
        )
    )]
    ItemPlantThermogenicGenepool1 = -177792789i32,
    #[strum(serialize = "StructureInsulatedInLineTankGas1x2")]
    #[strum(props(name = "Insulated In-Line Tank Gas", desc = "", value = "-177610944"))]
    StructureInsulatedInLineTankGas1X2 = -177610944i32,
    #[strum(serialize = "StructureCableCornerBurnt")]
    #[strum(props(name = "Burnt Cable (Corner)", desc = "", value = "-177220914"))]
    StructureCableCornerBurnt = -177220914i32,
    #[strum(serialize = "StructureCableJunction")]
    #[strum(
        props(
            name = "Cable (Junction)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life - so much so, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link> during the 3rd Decannual Stationeer Solar Conference.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "-175342021"
        )
    )]
    StructureCableJunction = -175342021i32,
    #[strum(serialize = "ItemKitLaunchTower")]
    #[strum(props(name = "Kit (Rocket Launch Tower)", desc = "", value = "-174523552"))]
    ItemKitLaunchTower = -174523552i32,
    #[strum(serialize = "StructureBench3")]
    #[strum(props(name = "Bench (Frame Style)", desc = "", value = "-164622691"))]
    StructureBench3 = -164622691i32,
    #[strum(serialize = "MotherboardProgrammableChip")]
    #[strum(
        props(
            name = "IC Editor Motherboard",
            desc = "When placed in a <link=ThingStructureComputer><color=green>Computer</color></link>, the IC Editor allows players to write and edit IC code, which can then be uploaded to a <link=ThingItemIntegratedCircuit10><color=green>Integrated Circuit (IC10)</color></link> if housed in an <link=ThingStructureCircuitHousing><color=green>IC Housing</color></link>.",
            value = "-161107071"
        )
    )]
    MotherboardProgrammableChip = -161107071i32,
    #[strum(serialize = "ItemSprayCanOrange")]
    #[strum(
        props(
            name = "Spray Paint (Orange)",
            desc = "Orange is fun, but also suggestive of hazards. Sitting proudly in the middle of the visual spectrum, it has nothing to prove.",
            value = "-158007629"
        )
    )]
    ItemSprayCanOrange = -158007629i32,
    #[strum(serialize = "StructureWallPaddedCorner")]
    #[strum(props(name = "Wall (Padded Corner)", desc = "", value = "-155945899"))]
    StructureWallPaddedCorner = -155945899i32,
    #[strum(serialize = "StructureCableStraightH")]
    #[strum(props(name = "Heavy Cable (Straight)", desc = "", value = "-146200530"))]
    StructureCableStraightH = -146200530i32,
    #[strum(serialize = "StructureDockPortSide")]
    #[strum(props(name = "Dock (Port Side)", desc = "", value = "-137465079"))]
    StructureDockPortSide = -137465079i32,
    #[strum(serialize = "StructureCircuitHousing")]
    #[strum(props(name = "IC Housing", desc = "", value = "-128473777"))]
    StructureCircuitHousing = -128473777i32,
    #[strum(serialize = "MotherboardMissionControl")]
    #[strum(
        props(
            name = "<N:EN:MotherboardMissionControl>",
            desc = "<N:EN:MotherboardMissionControl>",
            value = "-127121474"
        )
    )]
    MotherboardMissionControl = -127121474i32,
    #[strum(serialize = "ItemKitSpeaker")]
    #[strum(props(name = "Kit (Speaker)", desc = "", value = "-126038526"))]
    ItemKitSpeaker = -126038526i32,
    #[strum(serialize = "StructureLogicReagentReader")]
    #[strum(props(name = "Reagent Reader", desc = "", value = "-124308857"))]
    StructureLogicReagentReader = -124308857i32,
    #[strum(serialize = "ItemGasFilterNitrousOxideInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Nitrous Oxide)",
            desc = "A filter that selectively targets Nitrous Oxide. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "-123934842"
        )
    )]
    ItemGasFilterNitrousOxideInfinite = -123934842i32,
    #[strum(serialize = "ItemKitPressureFedGasEngine")]
    #[strum(
        props(name = "Kit (Pressure Fed Gas Engine)", desc = "", value = "-121514007")
    )]
    ItemKitPressureFedGasEngine = -121514007i32,
    #[strum(serialize = "StructureCableJunction4HBurnt")]
    #[strum(
        props(name = "Burnt Cable (4-Way Junction)", desc = "", value = "-115809132")
    )]
    StructureCableJunction4HBurnt = -115809132i32,
    #[strum(serialize = "ElevatorCarrage")]
    #[strum(props(name = "Elevator", desc = "", value = "-110788403"))]
    ElevatorCarrage = -110788403i32,
    #[strum(serialize = "StructureFairingTypeA2")]
    #[strum(props(name = "Fairing (Type A2)", desc = "", value = "-104908736"))]
    StructureFairingTypeA2 = -104908736i32,
    #[strum(serialize = "ItemKitPressureFedLiquidEngine")]
    #[strum(
        props(name = "Kit (Pressure Fed Liquid Engine)", desc = "", value = "-99091572")
    )]
    ItemKitPressureFedLiquidEngine = -99091572i32,
    #[strum(serialize = "Meteorite")]
    #[strum(props(name = "Meteorite", desc = "", value = "-99064335"))]
    Meteorite = -99064335i32,
    #[strum(serialize = "ItemKitArcFurnace")]
    #[strum(props(name = "Kit (Arc Furnace)", desc = "", value = "-98995857"))]
    ItemKitArcFurnace = -98995857i32,
    #[strum(serialize = "StructureInsulatedPipeCrossJunction")]
    #[strum(
        props(
            name = "Insulated Pipe (Cross Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "-92778058"
        )
    )]
    StructureInsulatedPipeCrossJunction = -92778058i32,
    #[strum(serialize = "ItemWaterPipeMeter")]
    #[strum(props(name = "Kit (Liquid Pipe Meter)", desc = "", value = "-90898877"))]
    ItemWaterPipeMeter = -90898877i32,
    #[strum(serialize = "FireArmSMG")]
    #[strum(
        props(name = "Fire Arm SMG", desc = "0.Single\n1.Auto", value = "-86315541")
    )]
    FireArmSmg = -86315541i32,
    #[strum(serialize = "ItemHardsuitHelmet")]
    #[strum(
        props(
            name = "Hardsuit Helmet",
            desc = "The Hardsuit Helmet is similar to the <link=ThingItemSpaceHelmet><color=green>Space Helmet</color></link>, but can withstand higher temperatures and pressures. It's perfect for enduring harsh environments like Venus and Vulcan.",
            value = "-84573099"
        )
    )]
    ItemHardsuitHelmet = -84573099i32,
    #[strum(serialize = "ItemSolderIngot")]
    #[strum(props(name = "Ingot (Solder)", desc = "", value = "-82508479"))]
    ItemSolderIngot = -82508479i32,
    #[strum(serialize = "CircuitboardGasDisplay")]
    #[strum(
        props(
            name = "Gas Display",
            desc = "Information is power. Place this circuitboard into a <link=ThingStructureConsole><color=green>Console</color></link> to create a display that shows gas pressure or temperature of any connected tank, storage cannister, <link=ThingItemPipeAnalyizer><color=green>Kit (Pipe Analyzer)</color></link> or <link=ThingItemGasSensor><color=green>Kit (Gas Sensor)</color></link>.",
            value = "-82343730"
        )
    )]
    CircuitboardGasDisplay = -82343730i32,
    #[strum(serialize = "DynamicGenerator")]
    #[strum(
        props(
            name = "Portable Generator",
            desc = "Every <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> best friend, the portable generator gets you up and running, fast. Fill it with a <link=ThingItemGasCanisterFuel><color=green>Canister (Fuel)</color></link> to power up and charge a <link=ThingItemBatteryCell><color=green>Battery Cell (Small)</color></link>, or attach it to a <link=ThingStructurePowerConnector><color=green>Power Connector</color></link> to link it into your electrical network. It's pressure driven, so functions more efficiently at lower temperatures, and REALLY efficiently if supercooled. Perfecting your fuel mix also makes a big difference.",
            value = "-82087220"
        )
    )]
    DynamicGenerator = -82087220i32,
    #[strum(serialize = "ItemFlowerRed")]
    #[strum(props(name = "Flower (Red)", desc = "", value = "-81376085"))]
    ItemFlowerRed = -81376085i32,
    #[strum(serialize = "KitchenTableSimpleShort")]
    #[strum(
        props(name = "Kitchen Table (Simple Short)", desc = "", value = "-78099334")
    )]
    KitchenTableSimpleShort = -78099334i32,
    #[strum(serialize = "ImGuiCircuitboardAirlockControl")]
    #[strum(props(name = "Airlock (Experimental)", desc = "", value = "-73796547"))]
    ImGuiCircuitboardAirlockControl = -73796547i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidCrossJunction6")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (6-Way Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "-72748982"
        )
    )]
    StructureInsulatedPipeLiquidCrossJunction6 = -72748982i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCorner")]
    #[strum(
        props(
            name = "Composite Cladding (Angled Corner)",
            desc = "",
            value = "-69685069"
        )
    )]
    StructureCompositeCladdingAngledCorner = -69685069i32,
    #[strum(serialize = "StructurePowerTransmitter")]
    #[strum(
        props(
            name = "Microwave Power Transmitter",
            desc = "The <link=Norsec><color=#0080FFFF>Norsec</color></link> Wireless Power Transmitter is an uni-directional, A-to-B, far field microwave electrical transmission system.The rotatable base transmitter delivers a narrow, non-lethal microwave beam to a dedicated base receiver.\nThe transmitter must be aligned to the base station in order to transmit any power. The brightness of the transmitter's collimator arc provides an indication of transmission intensity. Note that there is an attrition over longer ranges, so the unit requires more power over greater distances to deliver the same output.",
            value = "-65087121"
        )
    )]
    StructurePowerTransmitter = -65087121i32,
    #[strum(serialize = "ItemFrenchFries")]
    #[strum(
        props(
            name = "Canned French Fries",
            desc = "Because space would suck without 'em.",
            value = "-57608687"
        )
    )]
    ItemFrenchFries = -57608687i32,
    #[strum(serialize = "StructureConsoleLED1x2")]
    #[strum(
        props(
            name = "LED Display (Medium)",
            desc = "0.Default\n1.Percent\n2.Power",
            value = "-53151617"
        )
    )]
    StructureConsoleLed1X2 = -53151617i32,
    #[strum(serialize = "UniformMarine")]
    #[strum(props(name = "Marine Uniform", desc = "", value = "-48342840"))]
    UniformMarine = -48342840i32,
    #[strum(serialize = "Battery_Wireless_cell_Big")]
    #[strum(
        props(
            name = "Battery Wireless Cell (Big)",
            desc = "0.Empty\n1.Critical\n2.VeryLow\n3.Low\n4.Medium\n5.High\n6.Full",
            value = "-41519077"
        )
    )]
    BatteryWirelessCellBig = -41519077i32,
    #[strum(serialize = "StructureCableCornerH")]
    #[strum(props(name = "Heavy Cable (Corner)", desc = "", value = "-39359015"))]
    StructureCableCornerH = -39359015i32,
    #[strum(serialize = "ItemPipeCowl")]
    #[strum(
        props(
            name = "Pipe Cowl",
            desc = "This creates a <link=ThingItemPipeCowl><color=green>Pipe Cowl</color></link> that can be placed on the end of pipes to expose them to the world atmospheres.",
            value = "-38898376"
        )
    )]
    ItemPipeCowl = -38898376i32,
    #[strum(serialize = "StructureStairwellFrontLeft")]
    #[strum(props(name = "Stairwell (Front Left)", desc = "", value = "-37454456"))]
    StructureStairwellFrontLeft = -37454456i32,
    #[strum(serialize = "StructureWallPaddedWindowThin")]
    #[strum(props(name = "Wall (Padded Window Thin)", desc = "", value = "-37302931"))]
    StructureWallPaddedWindowThin = -37302931i32,
    #[strum(serialize = "StructureInsulatedTankConnector")]
    #[strum(props(name = "Insulated Tank Connector", desc = "", value = "-31273349"))]
    StructureInsulatedTankConnector = -31273349i32,
    #[strum(serialize = "ItemKitInsulatedPipeUtility")]
    #[strum(
        props(name = "Kit (Insulated Pipe Utility Gas)", desc = "", value = "-27284803")
    )]
    ItemKitInsulatedPipeUtility = -27284803i32,
    #[strum(serialize = "DynamicLight")]
    #[strum(
        props(
            name = "Portable Light",
            desc = "Philippe Starck might not applaud, but this battery-powered light source undarkens the corners when illumination's lacking. Powered by any battery, it's a 'no-frills' <link=Xigo><color=#0080FFFF>Xigo</color></link> design that can be cheaply fabricated with the minimum of fuss. Unless you like fuss. In which case, fuss all you like.",
            value = "-21970188"
        )
    )]
    DynamicLight = -21970188i32,
    #[strum(serialize = "ItemKitBatteryLarge")]
    #[strum(props(name = "Kit (Battery Large)", desc = "", value = "-21225041"))]
    ItemKitBatteryLarge = -21225041i32,
    #[strum(serialize = "StructureSmallTableThickDouble")]
    #[strum(props(name = "Small (Table Thick Double)", desc = "", value = "-19246131"))]
    StructureSmallTableThickDouble = -19246131i32,
    #[strum(serialize = "ItemAmmoBox")]
    #[strum(props(name = "Ammo Box", desc = "", value = "-9559091"))]
    ItemAmmoBox = -9559091i32,
    #[strum(serialize = "StructurePipeLiquidCrossJunction4")]
    #[strum(
        props(
            name = "Liquid Pipe (4-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidCrossJunction4><color=green>Insulated Liquid Pipe (4-Way Junction)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "-9555593"
        )
    )]
    StructurePipeLiquidCrossJunction4 = -9555593i32,
    #[strum(serialize = "DynamicGasCanisterRocketFuel")]
    #[strum(
        props(name = "Dynamic Gas Canister Rocket Fuel", desc = "", value = "-8883951")
    )]
    DynamicGasCanisterRocketFuel = -8883951i32,
    #[strum(serialize = "ItemPureIcePollutant")]
    #[strum(
        props(
            name = "Pure Ice Pollutant",
            desc = "A frozen chunk of pure <link=GasPollutant><color=#44AD83>Pollutant</color></link>",
            value = "-1755356"
        )
    )]
    ItemPureIcePollutant = -1755356i32,
    #[strum(serialize = "ItemWreckageLargeExtendableRadiator01")]
    #[strum(
        props(name = "Wreckage Large Extendable Radiator", desc = "", value = "-997763")
    )]
    ItemWreckageLargeExtendableRadiator01 = -997763i32,
    #[strum(serialize = "StructureSingleBed")]
    #[strum(props(name = "Single Bed", desc = "Description coming.", value = "-492611"))]
    StructureSingleBed = -492611i32,
    #[strum(serialize = "StructureCableCorner3HBurnt")]
    #[strum(
        props(
            name = "<N:EN:StructureCableCorner3HBurnt>",
            desc = "<N:EN:StructureCableCorner3HBurnt>",
            value = "2393826"
        )
    )]
    StructureCableCorner3HBurnt = 2393826i32,
    #[strum(serialize = "StructureAutoMinerSmall")]
    #[strum(
        props(
            name = "Autominer (Small)",
            desc = "The <link=Recurso><color=#0080FFFF>Recurso</color></link> SquareDig autominer is a structure that when built will mine a vertical 2x2 shaft until it hits bedrock. The autominer can be connected to a chute system, and is controllable by a logic network. Note that the autominer outputs more <link=OrePage><color=#0080FFFF>ore</color></link> than a conventional <link=ThingItemMiningDrill><color=green>Mining Drill</color></link> over the same area.",
            value = "7274344"
        )
    )]
    StructureAutoMinerSmall = 7274344i32,
    #[strum(serialize = "CrateMkII")]
    #[strum(
        props(
            name = "Crate Mk II",
            desc = "A more heavily reinforced version of the iconic <link=ThingDynamicCrate><color=green>Dynamic Crate</color></link>, the Crate Mk II is resistant to incredibly high pressures and temperatures. Short of disposing of it in a black hole, the Mk II is about as safe as luggage gets.",
            value = "8709219"
        )
    )]
    CrateMkIi = 8709219i32,
    #[strum(serialize = "ItemGasFilterWaterM")]
    #[strum(props(name = "Medium Filter (Water)", desc = "", value = "8804422"))]
    ItemGasFilterWaterM = 8804422i32,
    #[strum(serialize = "StructureWallPaddedNoBorder")]
    #[strum(props(name = "Wall (Padded No Border)", desc = "", value = "8846501"))]
    StructureWallPaddedNoBorder = 8846501i32,
    #[strum(serialize = "ItemGasFilterVolatiles")]
    #[strum(
        props(
            name = "Filter (Volatiles)",
            desc = "Filters are used to capture various gases, which can be disposed of or used elsewhere. <link=GasVolatiles><color=#44AD83>Volatiles</color></link> are created by exposing <link=ThingItemVolatiles><color=green>Ice (Volatiles)</color></link> to heat. The product can then be collected and combined with <link=GasOxygen><color=#44AD83>Oxygen</color></link> to create fuel, or used within a <link=ThingStructureFurnace><color=green>Furnace</color></link> to smelt ores and create <link=IngotPage><color=#0080FFFF>alloys</color></link>.",
            value = "15011598"
        )
    )]
    ItemGasFilterVolatiles = 15011598i32,
    #[strum(serialize = "ItemMiningCharge")]
    #[strum(
        props(
            name = "Mining Charge",
            desc = "A low cost, high yield explosive with a 10 second timer.",
            value = "15829510"
        )
    )]
    ItemMiningCharge = 15829510i32,
    #[strum(serialize = "ItemKitEngineSmall")]
    #[strum(props(name = "Kit (Engine Small)", desc = "", value = "19645163"))]
    ItemKitEngineSmall = 19645163i32,
    #[strum(serialize = "StructureHeatExchangerGastoGas")]
    #[strum(
        props(
            name = "Heat Exchanger - Gas",
            desc = "The original specs for the N Series Flow-P heat exchanger were rumored to have been scrawled on the back of a burger receipt by a bored <link=Sinotai><color=#0080FFFF>Sinotai</color></link> designer riding up the Brazilian space elevator, but that hasn't stopped it becoming one of the most widely-copied heat exchanger designs in the Solar System.\nThe 'N Flow-P' has four connections, allowing you to pass two gas networks into the unit, which then works to equalize temperature across the two separate networks.\nAs the N Flow-P is a passive system, it equalizes pressure across the entire of each individual network, unless connected to gas management devices like a <link=ThingStructureVolumePump><color=green>Volume Pump</color></link> or a <link=ThingStructureBackPressureRegulator><color=green>Back Pressure Regulator</color></link>.",
            value = "21266291"
        )
    )]
    StructureHeatExchangerGastoGas = 21266291i32,
    #[strum(serialize = "StructurePressurantValve")]
    #[strum(
        props(
            name = "Pressurant Valve",
            desc = "Pumps gas into a liquid pipe in order to raise the pressure",
            value = "23052817"
        )
    )]
    StructurePressurantValve = 23052817i32,
    #[strum(serialize = "StructureWallHeater")]
    #[strum(
        props(
            name = "Wall Heater",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> wall heater is a simple device that can be installed on a wall or frame and connected to power. When switched on, it will start heating the surrounding environment. It consumes 1010W of power and can be controlled by logic chips to run when the temperature hits a certain level.",
            value = "24258244"
        )
    )]
    StructureWallHeater = 24258244i32,
    #[strum(serialize = "StructurePassiveLargeRadiatorLiquid")]
    #[strum(
        props(
            name = "Medium Convection Radiator Liquid",
            desc = "Has been replaced by <link=ThingStructureMediumConvectionRadiatorLiquid><color=green>Medium Convection Radiator Liquid</color></link>.",
            value = "24786172"
        )
    )]
    StructurePassiveLargeRadiatorLiquid = 24786172i32,
    #[strum(serialize = "StructureWallPlating")]
    #[strum(props(name = "Wall (Plating)", desc = "", value = "26167457"))]
    StructureWallPlating = 26167457i32,
    #[strum(serialize = "ItemSprayCanPurple")]
    #[strum(
        props(
            name = "Spray Paint (Purple)",
            desc = "Purple is a curious color. You need to be careful with purple. It can be very good, or go horribly, horribly wrong.",
            value = "30686509"
        )
    )]
    ItemSprayCanPurple = 30686509i32,
    #[strum(serialize = "DynamicGasCanisterNitrousOxide")]
    #[strum(
        props(name = "Portable Gas Tank (Nitrous Oxide)", desc = "", value = "30727200")
    )]
    DynamicGasCanisterNitrousOxide = 30727200i32,
    #[strum(serialize = "StructureInLineTankGas1x2")]
    #[strum(
        props(
            name = "In-Line Tank Gas",
            desc = "A small expansion tank that increases the volume of a pipe network.",
            value = "35149429"
        )
    )]
    StructureInLineTankGas1X2 = 35149429i32,
    #[strum(serialize = "ItemSteelSheets")]
    #[strum(
        props(
            name = "Steel Sheets",
            desc = "An advanced building material, <link=ThingItemSteelIngot><color=green>Ingot (Steel)</color></link> sheets are used when constructing a <link=ThingStructureFrame><color=green>Steel Frame</color></link> and several other <link=StructuresPage><color=#0080FFFF>wall</color></link> types.",
            value = "38555961"
        )
    )]
    ItemSteelSheets = 38555961i32,
    #[strum(serialize = "ItemGasCanisterEmpty")]
    #[strum(props(name = "Canister", desc = "", value = "42280099"))]
    ItemGasCanisterEmpty = 42280099i32,
    #[strum(serialize = "ItemWreckageWallCooler2")]
    #[strum(props(name = "Wreckage Wall Cooler", desc = "", value = "45733800"))]
    ItemWreckageWallCooler2 = 45733800i32,
    #[strum(serialize = "ItemPumpkinPie")]
    #[strum(props(name = "Pumpkin Pie", desc = "", value = "62768076"))]
    ItemPumpkinPie = 62768076i32,
    #[strum(serialize = "ItemGasFilterPollutantsM")]
    #[strum(props(name = "Medium Filter (Pollutants)", desc = "", value = "63677771"))]
    ItemGasFilterPollutantsM = 63677771i32,
    #[strum(serialize = "StructurePipeStraight")]
    #[strum(
        props(
            name = "Pipe (Straight)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeStraight><color=green>Insulated Pipe (Straight)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "73728932"
        )
    )]
    StructurePipeStraight = 73728932i32,
    #[strum(serialize = "ItemKitDockingPort")]
    #[strum(props(name = "Kit (Docking Port)", desc = "", value = "77421200"))]
    ItemKitDockingPort = 77421200i32,
    #[strum(serialize = "CartridgeTracker")]
    #[strum(props(name = "Tracker", desc = "", value = "81488783"))]
    CartridgeTracker = 81488783i32,
    #[strum(serialize = "ToyLuna")]
    #[strum(props(name = "Toy Luna", desc = "", value = "94730034"))]
    ToyLuna = 94730034i32,
    #[strum(serialize = "ItemWreckageTurbineGenerator2")]
    #[strum(props(name = "Wreckage Turbine Generator", desc = "", value = "98602599"))]
    ItemWreckageTurbineGenerator2 = 98602599i32,
    #[strum(serialize = "StructurePowerUmbilicalFemale")]
    #[strum(props(name = "Umbilical Socket (Power)", desc = "", value = "101488029"))]
    StructurePowerUmbilicalFemale = 101488029i32,
    #[strum(serialize = "DynamicSkeleton")]
    #[strum(props(name = "Skeleton", desc = "", value = "106953348"))]
    DynamicSkeleton = 106953348i32,
    #[strum(serialize = "ItemWaterBottle")]
    #[strum(
        props(
            name = "Water Bottle",
            desc = "Delicious and pure H20, refined from local sources as varied as Venusian ice and trans-Solar comets. Empty bottles can be refilled using the <link=ThingStructureWaterBottleFiller><color=green>Water Bottle Filler</color></link>.",
            value = "107741229"
        )
    )]
    ItemWaterBottle = 107741229i32,
    #[strum(serialize = "DynamicGasCanisterVolatiles")]
    #[strum(
        props(
            name = "Portable Gas Tank (Volatiles)",
            desc = "Portable tanks store gas. To refill one, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link>, then connect it to a pipe network. Don't fill it above 10 MPa, unless you're the sort who loves complicated, flammable emergencies. You can refill a <link=ThingItemGasCanisterVolatiles><color=green>Canister (Volatiles)</color></link> by attaching it to the tank's striped section. Or you could use a <link=ThingItemWrench><color=green>Wrench</color></link> to attach to a rocket and show it around the Solar System.",
            value = "108086870"
        )
    )]
    DynamicGasCanisterVolatiles = 108086870i32,
    #[strum(serialize = "StructureCompositeCladdingRoundedCornerInner")]
    #[strum(
        props(
            name = "Composite Cladding (Rounded Corner Inner)",
            desc = "",
            value = "110184667"
        )
    )]
    StructureCompositeCladdingRoundedCornerInner = 110184667i32,
    #[strum(serialize = "ItemTerrainManipulator")]
    #[strum(
        props(
            name = "Terrain Manipulator",
            desc = "0.Mode0\n1.Mode1",
            value = "111280987"
        )
    )]
    ItemTerrainManipulator = 111280987i32,
    #[strum(serialize = "FlareGun")]
    #[strum(props(name = "Flare Gun", desc = "", value = "118685786"))]
    FlareGun = 118685786i32,
    #[strum(serialize = "ItemKitPlanter")]
    #[strum(props(name = "Kit (Planter)", desc = "", value = "119096484"))]
    ItemKitPlanter = 119096484i32,
    #[strum(serialize = "ReagentColorGreen")]
    #[strum(props(name = "Color Dye (Green)", desc = "", value = "120807542"))]
    ReagentColorGreen = 120807542i32,
    #[strum(serialize = "DynamicGasCanisterNitrogen")]
    #[strum(
        props(
            name = "Portable Gas Tank (Nitrogen)",
            desc = "Portable tanks store gas. If you need to refill a tank, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or you'll end up with <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> in places you weren't expecting. You can refill a <link=ThingItemGasCanisterNitrogen><color=green>Canister (Nitrogen)</color></link> by attaching it to the tank's striped section. Or you could use a <link=ThingItemWrench><color=green>Wrench</color></link> to attach it to a rover or rocket for later.",
            value = "121951301"
        )
    )]
    DynamicGasCanisterNitrogen = 121951301i32,
    #[strum(serialize = "ItemKitPressurePlate")]
    #[strum(props(name = "Kit (Trigger Plate)", desc = "", value = "123504691"))]
    ItemKitPressurePlate = 123504691i32,
    #[strum(serialize = "ItemKitLogicSwitch")]
    #[strum(props(name = "Kit (Logic Switch)", desc = "", value = "124499454"))]
    ItemKitLogicSwitch = 124499454i32,
    #[strum(serialize = "StructureCompositeCladdingSpherical")]
    #[strum(
        props(name = "Composite Cladding (Spherical)", desc = "", value = "139107321")
    )]
    StructureCompositeCladdingSpherical = 139107321i32,
    #[strum(serialize = "ItemLaptop")]
    #[strum(
        props(
            name = "Laptop",
            desc = "The Laptop functions as a portable IC editor. To operate the Laptop it must be powered with a battery, have a <link=ThingMotherboardProgrammableChip><color=green>IC Editor Motherboard</color></link> in the motherboard slot, and an <link=ThingItemIntegratedCircuit10><color=green>Integrated Circuit (IC10)</color></link> in the Programmable Chip Slot.\n\nYou must place the laptop down to interact with the onsreen UI.\n        \nConnects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "141535121"
        )
    )]
    ItemLaptop = 141535121i32,
    #[strum(serialize = "ApplianceSeedTray")]
    #[strum(
        props(
            name = "Appliance Seed Tray",
            desc = "The seed tray can hold up to twelve plants or seeds and can be used to facilitate fast experimentation and testing of plant genetics.",
            value = "142831994"
        )
    )]
    ApplianceSeedTray = 142831994i32,
    #[strum(serialize = "Landingpad_TaxiPieceHold")]
    #[strum(props(name = "Landingpad Taxi Hold", desc = "", value = "146051619"))]
    LandingpadTaxiPieceHold = 146051619i32,
    #[strum(serialize = "StructureFuselageTypeC5")]
    #[strum(props(name = "Fuselage (Type C5)", desc = "", value = "147395155"))]
    StructureFuselageTypeC5 = 147395155i32,
    #[strum(serialize = "ItemKitBasket")]
    #[strum(props(name = "Kit (Basket)", desc = "", value = "148305004"))]
    ItemKitBasket = 148305004i32,
    #[strum(serialize = "StructureRocketCircuitHousing")]
    #[strum(props(name = "Rocket Circuit Housing", desc = "", value = "150135861"))]
    StructureRocketCircuitHousing = 150135861i32,
    #[strum(serialize = "StructurePipeCrossJunction6")]
    #[strum(
        props(
            name = "Pipe (6-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCrossJunction6><color=green>Insulated Pipe (6-Way Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "152378047"
        )
    )]
    StructurePipeCrossJunction6 = 152378047i32,
    #[strum(serialize = "ItemGasFilterNitrogenInfinite")]
    #[strum(
        props(
            name = "Catalytic Filter (Nitrogen)",
            desc = "A filter that selectively targets Nitrogen. It uses internal pressure differentials to regenerate a unique phase change catalyst, giving it an unlimited lifecycle.",
            value = "152751131"
        )
    )]
    ItemGasFilterNitrogenInfinite = 152751131i32,
    #[strum(serialize = "StructureStairs4x2RailL")]
    #[strum(props(name = "Stairs with Rail (Left)", desc = "", value = "155214029"))]
    StructureStairs4X2RailL = 155214029i32,
    #[strum(serialize = "NpcChick")]
    #[strum(props(name = "Chick", desc = "", value = "155856647"))]
    NpcChick = 155856647i32,
    #[strum(serialize = "ItemWaspaloyIngot")]
    #[strum(props(name = "Ingot (Waspaloy)", desc = "", value = "156348098"))]
    ItemWaspaloyIngot = 156348098i32,
    #[strum(serialize = "StructureReinforcedWallPaddedWindowThin")]
    #[strum(
        props(
            name = "Reinforced Window (Thin)",
            desc = "Enjoy vistas of even the most savage, alien landscapes with these heavy duty window frames, which are resistant to pressure differentials up to 1MPa.",
            value = "158502707"
        )
    )]
    StructureReinforcedWallPaddedWindowThin = 158502707i32,
    #[strum(serialize = "ItemKitWaterBottleFiller")]
    #[strum(props(name = "Kit (Water Bottle Filler)", desc = "", value = "159886536"))]
    ItemKitWaterBottleFiller = 159886536i32,
    #[strum(serialize = "ItemEmergencyWrench")]
    #[strum(props(name = "Emergency Wrench", desc = "", value = "162553030"))]
    ItemEmergencyWrench = 162553030i32,
    #[strum(serialize = "StructureChuteDigitalFlipFlopSplitterRight")]
    #[strum(
        props(
            name = "Chute Digital Flip Flop Splitter Right",
            desc = "The digital flip flop will toggle between two outputs using a specified ratio (n:1). For example, setting the dial to 2 would allow two items to pass through the primary output before flipping to the secondary output.",
            value = "163728359"
        )
    )]
    StructureChuteDigitalFlipFlopSplitterRight = 163728359i32,
    #[strum(serialize = "StructureChuteStraight")]
    #[strum(
        props(
            name = "Chute (Straight)",
            desc = "Chutes act as pipes for items. Use them to connect various <link=ImportExportPage><color=#0080FFFF>import/export</color></link> equipment together such as the <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link> and printers like the <link=ThingStructureAutolathe><color=green>Autolathe</color></link>.\nThe aim for any <link=Stationeers><color=#0080FFFF>Stationeer</color></link> is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nChutes are fundamental components of chute networks, which allow the transport of items between any machine or device with an <link=ImportExportPage><color=#0080FFFF>import/export</color></link> slot.",
            value = "168307007"
        )
    )]
    StructureChuteStraight = 168307007i32,
    #[strum(serialize = "ItemKitDoor")]
    #[strum(props(name = "Kit (Door)", desc = "", value = "168615924"))]
    ItemKitDoor = 168615924i32,
    #[strum(serialize = "ItemWreckageAirConditioner2")]
    #[strum(props(name = "Wreckage Air Conditioner", desc = "", value = "169888054"))]
    ItemWreckageAirConditioner2 = 169888054i32,
    #[strum(serialize = "Landingpad_GasCylinderTankPiece")]
    #[strum(
        props(
            name = "Landingpad Gas Storage",
            desc = "Increases the volume of the landing pads gas storage capacity. This volume is used for buying and selling gas to traders.",
            value = "170818567"
        )
    )]
    LandingpadGasCylinderTankPiece = 170818567i32,
    #[strum(serialize = "ItemKitStairs")]
    #[strum(props(name = "Kit (Stairs)", desc = "", value = "170878959"))]
    ItemKitStairs = 170878959i32,
    #[strum(serialize = "ItemPlantSampler")]
    #[strum(
        props(
            name = "Plant Sampler",
            desc = "The Plant Sampler allows you to take a <link=GeneticsPage><color=#0080FFFF>gene</color></link> sample of a growing plant. The sampler can then be placed in the <link=ThingAppliancePlantGeneticAnalyzer><color=green>Plant Genetic Analyzer</color></link> to attain and interpret the results.",
            value = "173023800"
        )
    )]
    ItemPlantSampler = 173023800i32,
    #[strum(serialize = "ItemAlienMushroom")]
    #[strum(props(name = "Alien Mushroom", desc = "", value = "176446172"))]
    ItemAlienMushroom = 176446172i32,
    #[strum(serialize = "ItemKitSatelliteDish")]
    #[strum(props(name = "Kit (Medium Satellite Dish)", desc = "", value = "178422810"))]
    ItemKitSatelliteDish = 178422810i32,
    #[strum(serialize = "StructureRocketEngineTiny")]
    #[strum(props(name = "Rocket Engine (Tiny)", desc = "", value = "178472613"))]
    StructureRocketEngineTiny = 178472613i32,
    #[strum(serialize = "StructureWallPaddedNoBorderCorner")]
    #[strum(
        props(name = "Wall (Padded No Border Corner)", desc = "", value = "179694804")
    )]
    StructureWallPaddedNoBorderCorner = 179694804i32,
    #[strum(serialize = "StructureShelfMedium")]
    #[strum(
        props(
            name = "Shelf Medium",
            desc = "A shelf for putting things on, so you can see them.",
            value = "182006674"
        )
    )]
    StructureShelfMedium = 182006674i32,
    #[strum(serialize = "StructureExpansionValve")]
    #[strum(
        props(
            name = "Expansion Valve",
            desc = "Allows for moving liquids from a liquid pipe into a gas pipe. Only allows liquids to pass in one direction. Typically this is done to allow the liquid to evaporate into a gas as part of an airconditioning loop.",
            value = "195298587"
        )
    )]
    StructureExpansionValve = 195298587i32,
    #[strum(serialize = "ItemCableFuse")]
    #[strum(props(name = "Kit (Cable Fuses)", desc = "", value = "195442047"))]
    ItemCableFuse = 195442047i32,
    #[strum(serialize = "ItemKitRoverMKI")]
    #[strum(props(name = "Kit (Rover Mk I)", desc = "", value = "197243872"))]
    ItemKitRoverMki = 197243872i32,
    #[strum(serialize = "DynamicGasCanisterWater")]
    #[strum(
        props(
            name = "Portable Liquid Tank (Water)",
            desc = "This portable tank stores liquid, and liquid only. You just have to fill it up. To do this, bolt one to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link>, then connect it to <link=ThingStructurePipeLiquidStraight><color=green>Liquid Pipe (Straight)</color></link> to supply liquid to a network. \nTry to keep pressure under 10 MPa, or you'll end up wet, hurt and sorry, without any of the fun.\nYou can refill a <link=ThingItemGasCanisterWater><color=green>Liquid Canister (Water)</color></link> by attaching it to the tank's striped section. Or you could use a <link=ThingItemWrench><color=green>Wrench</color></link> to attach it to a rocket and take it somewhere distant and dry, then feel good about yourself.",
            value = "197293625"
        )
    )]
    DynamicGasCanisterWater = 197293625i32,
    #[strum(serialize = "ItemAngleGrinder")]
    #[strum(
        props(
            name = "Angle Grinder",
            desc = "Angles-be-gone with the trusty angle grinder.",
            value = "201215010"
        )
    )]
    ItemAngleGrinder = 201215010i32,
    #[strum(serialize = "StructureCableCornerH4")]
    #[strum(props(name = "Heavy Cable (4-Way Corner)", desc = "", value = "205837861"))]
    StructureCableCornerH4 = 205837861i32,
    #[strum(serialize = "ItemEmergencySpaceHelmet")]
    #[strum(props(name = "Emergency Space Helmet", desc = "", value = "205916793"))]
    ItemEmergencySpaceHelmet = 205916793i32,
    #[strum(serialize = "ItemKitGovernedGasRocketEngine")]
    #[strum(
        props(name = "Kit (Pumped Gas Rocket Engine)", desc = "", value = "206848766")
    )]
    ItemKitGovernedGasRocketEngine = 206848766i32,
    #[strum(serialize = "StructurePressureRegulator")]
    #[strum(
        props(
            name = "Pressure Regulator",
            desc = "Controlling the flow of gas between two pipe networks, pressure regulators shift gas until a set pressure on the outlet side is achieved, or the gas supply is exhausted. The back pressure regulator, by contrast, will only operate when pressure on the intake side exceeds the set value. With a max pressure of over 20,000kPa, it requires power to operate.",
            value = "209854039"
        )
    )]
    StructurePressureRegulator = 209854039i32,
    #[strum(serialize = "StructureCompositeCladdingCylindrical")]
    #[strum(
        props(name = "Composite Cladding (Cylindrical)", desc = "", value = "212919006")
    )]
    StructureCompositeCladdingCylindrical = 212919006i32,
    #[strum(serialize = "ItemCropHay")]
    #[strum(props(name = "Hay", desc = "", value = "215486157"))]
    ItemCropHay = 215486157i32,
    #[strum(serialize = "ItemKitLogicProcessor")]
    #[strum(props(name = "Kit (Logic Processor)", desc = "", value = "220644373"))]
    ItemKitLogicProcessor = 220644373i32,
    #[strum(serialize = "AutolathePrinterMod")]
    #[strum(
        props(
            name = "Autolathe Printer Mod",
            desc = "Apply to an <link=ThingStructureAutolathe><color=green>Autolathe</color></link> with a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> or <link=ThingItemArcWelder><color=green>Arc Welder</color></link> to upgrade for increased processing speed and more recipe options.",
            value = "221058307"
        )
    )]
    AutolathePrinterMod = 221058307i32,
    #[strum(serialize = "StructureChuteOverflow")]
    #[strum(
        props(
            name = "Chute Overflow",
            desc = "The overflow chute will direct materials to its overflow port when the thing connected to its default port is already occupied.",
            value = "225377225"
        )
    )]
    StructureChuteOverflow = 225377225i32,
    #[strum(serialize = "ItemLiquidPipeAnalyzer")]
    #[strum(props(name = "Kit (Liquid Pipe Analyzer)", desc = "", value = "226055671"))]
    ItemLiquidPipeAnalyzer = 226055671i32,
    #[strum(serialize = "ItemGoldIngot")]
    #[strum(
        props(
            name = "Ingot (Gold)",
            desc = "There is an enduring paradox at the heart of the <link=Stationeers><color=#0080FFFF>Stationeers</color></link> project: An initiative conceived as 'cut-price space exploration' uses <link=ReagentGold><color=#B566FF>Gold</color></link> as a fundamental ingredient in fabricating so much of its equipment and materiel. ",
            value = "226410516"
        )
    )]
    ItemGoldIngot = 226410516i32,
    #[strum(serialize = "KitStructureCombustionCentrifuge")]
    #[strum(props(name = "Kit (Combustion Centrifuge)", desc = "", value = "231903234"))]
    KitStructureCombustionCentrifuge = 231903234i32,
    #[strum(serialize = "ItemChocolateBar")]
    #[strum(props(name = "Chocolate Bar", desc = "", value = "234601764"))]
    ItemChocolateBar = 234601764i32,
    #[strum(serialize = "ItemExplosive")]
    #[strum(props(name = "Remote Explosive", desc = "", value = "235361649"))]
    ItemExplosive = 235361649i32,
    #[strum(serialize = "StructureConsole")]
    #[strum(
        props(
            name = "Console",
            desc = "This <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> control box manages devices such as the <link=ThingStructureActiveVent><color=green>Active Vent</color></link>, <link=ThingStructurePassiveVent><color=green>Passive Vent</color></link>, <link=ThingStructureGasSensor><color=green>Gas Sensor</color></link> and <link=ThingStructureCompositeDoor><color=green>Composite Door</color></link>, depending on which <link=LogicPage><color=#0080FFFF>circuitboard</color></link> is inserted into the unit. It has a shared data/power port.\nA completed console displays all devices connected to the current power network. Any devices not related to the installed circuitboard will be greyed-out and inoperable. Consoles are locked once a <link=ThingItemDataDisk><color=green>Data Disk</color></link> is removed.",
            value = "235638270"
        )
    )]
    StructureConsole = 235638270i32,
    #[strum(serialize = "ItemPassiveVent")]
    #[strum(
        props(
            name = "Passive Vent",
            desc = "This kit creates a <link=ThingItemPassiveVent><color=green>Passive Vent</color></link> among other variants.",
            value = "238631271"
        )
    )]
    ItemPassiveVent = 238631271i32,
    #[strum(serialize = "ItemMKIIAngleGrinder")]
    #[strum(
        props(
            name = "Mk II Angle Grinder",
            desc = "Angles-be-gone with the trusty angle grinder. The MK II is more resistant to temperature and pressure.",
            value = "240174650"
        )
    )]
    ItemMkiiAngleGrinder = 240174650i32,
    #[strum(serialize = "Handgun")]
    #[strum(props(name = "Handgun", desc = "", value = "247238062"))]
    Handgun = 247238062i32,
    #[strum(serialize = "PassiveSpeaker")]
    #[strum(props(name = "Passive Speaker", desc = "", value = "248893646"))]
    PassiveSpeaker = 248893646i32,
    #[strum(serialize = "ItemKitBeacon")]
    #[strum(props(name = "Kit (Beacon)", desc = "", value = "249073136"))]
    ItemKitBeacon = 249073136i32,
    #[strum(serialize = "ItemCharcoal")]
    #[strum(
        props(
            name = "Charcoal",
            desc = "Charcoal is a lightweight, black carbon residue produced by heating <link=ThingItemBiomass><color=green>Biomass</color></link> in a <link=ThingStructureArcFurnace><color=green>Arc Furnace</color></link>. It contains less energy potential than <link=ThingItemCoalOre><color=green>Ore (Coal)</color></link>, but can be used as a basic fuel source. Charcoal can also be substituted for coal in <link=IngotPage><color=#0080FFFF>alloy</color></link> recipes.",
            value = "252561409"
        )
    )]
    ItemCharcoal = 252561409i32,
    #[strum(serialize = "StructureSuitStorage")]
    #[strum(
        props(
            name = "Suit Storage",
            desc = "As tidy as it is useful, the suit storage rack holds an <link=ThingItemEvaSuit><color=green>Eva Suit</color></link>, <link=ThingItemSpaceHelmet><color=green>Space Helmet</color></link> and a <link=ThingItemJetpackBasic><color=green>Jetpack Basic</color></link>.\nWhen powered and connected to <link=ThingOxygen><color=green><N:EN:Oxygen></color></link> and <link=ThingPropellant><color=green><N:EN:Propellant></color></link>, it will recharge the suit's batteries, refill the <link=ThingItemGasCanisterOxygen><color=green>Canister (Oxygen)</color></link> and your <link=ThingItemGasFilterNitrogen><color=green>Filter (Nitrogen)</color></link> <link=SlotGasCanister><color=orange>Gas Canister</color></link>. The wastetank will be pumped out to the pipe connected to the waste outlet.\nAll the rack's pipes must be connected or the unit will show an error state, but it will still charge the battery.",
            value = "255034731"
        )
    )]
    StructureSuitStorage = 255034731i32,
    #[strum(serialize = "ItemCorn")]
    #[strum(
        props(
            name = "Corn",
            desc = "A long growth time staple crop. Its low requirement for darkness allows for accelerated growing if provided with extra light.",
            value = "258339687"
        )
    )]
    ItemCorn = 258339687i32,
    #[strum(serialize = "StructurePipeLiquidTJunction")]
    #[strum(
        props(
            name = "Liquid Pipe (T Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidTJunction><color=green>Insulated Liquid Pipe (T Junction)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "262616717"
        )
    )]
    StructurePipeLiquidTJunction = 262616717i32,
    #[strum(serialize = "StructureLogicBatchReader")]
    #[strum(props(name = "Batch Reader", desc = "", value = "264413729"))]
    StructureLogicBatchReader = 264413729i32,
    #[strum(serialize = "StructureDeepMiner")]
    #[strum(
        props(
            name = "Deep Miner",
            desc = "Drills through terrain until it hits bedrock. Once inside bedrock <link=ThingItemDirtyOre><color=green>Dirty Ore</color></link> is produced roughly every 90s",
            value = "265720906"
        )
    )]
    StructureDeepMiner = 265720906i32,
    #[strum(serialize = "ItemEmergencyScrewdriver")]
    #[strum(props(name = "Emergency Screwdriver", desc = "", value = "266099983"))]
    ItemEmergencyScrewdriver = 266099983i32,
    #[strum(serialize = "ItemFilterFern")]
    #[strum(
        props(
            name = "Darga Fern",
            desc = "A fern adapted by <link=Agrizero><color=#0080FFFF>Agrizero</color></link>to process a much greater volume of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> into <link=GasOxygen><color=#44AD83>Oxygen</color></link> than an average plant.",
            value = "266654416"
        )
    )]
    ItemFilterFern = 266654416i32,
    #[strum(serialize = "StructureCableCorner4Burnt")]
    #[strum(props(name = "Burnt Cable (4-Way Corner)", desc = "", value = "268421361"))]
    StructureCableCorner4Burnt = 268421361i32,
    #[strum(serialize = "StructureFrameCornerCut")]
    #[strum(
        props(
            name = "Steel Frame (Corner Cut)",
            desc = "0.Mode0\n1.Mode1",
            value = "271315669"
        )
    )]
    StructureFrameCornerCut = 271315669i32,
    #[strum(serialize = "StructureTankSmallInsulated")]
    #[strum(props(name = "Tank Small (Insulated)", desc = "", value = "272136332"))]
    StructureTankSmallInsulated = 272136332i32,
    #[strum(serialize = "StructureCableFuse100k")]
    #[strum(props(name = "Fuse (100kW)", desc = "", value = "281380789"))]
    StructureCableFuse100K = 281380789i32,
    #[strum(serialize = "ItemKitIceCrusher")]
    #[strum(props(name = "Kit (Ice Crusher)", desc = "", value = "288111533"))]
    ItemKitIceCrusher = 288111533i32,
    #[strum(serialize = "ItemKitPowerTransmitter")]
    #[strum(props(name = "Kit (Power Transmitter)", desc = "", value = "291368213"))]
    ItemKitPowerTransmitter = 291368213i32,
    #[strum(serialize = "StructurePipeLiquidCrossJunction6")]
    #[strum(
        props(
            name = "Liquid Pipe (6-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidCrossJunction6><color=green>Insulated Liquid Pipe (6-Way Junction)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "291524699"
        )
    )]
    StructurePipeLiquidCrossJunction6 = 291524699i32,
    #[strum(serialize = "ItemKitLandingPadBasic")]
    #[strum(props(name = "Kit (Landing Pad Basic)", desc = "", value = "293581318"))]
    ItemKitLandingPadBasic = 293581318i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidStraight")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (Straight)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "295678685"
        )
    )]
    StructureInsulatedPipeLiquidStraight = 295678685i32,
    #[strum(serialize = "StructureWallFlatCornerSquare")]
    #[strum(props(name = "Wall (Flat Corner Square)", desc = "", value = "298130111"))]
    StructureWallFlatCornerSquare = 298130111i32,
    #[strum(serialize = "ItemHat")]
    #[strum(
        props(
            name = "Hat",
            desc = "As the name suggests, this is a hat.",
            value = "299189339"
        )
    )]
    ItemHat = 299189339i32,
    #[strum(serialize = "ItemWaterPipeDigitalValve")]
    #[strum(props(name = "Kit (Liquid Digital Valve)", desc = "", value = "309693520"))]
    ItemWaterPipeDigitalValve = 309693520i32,
    #[strum(serialize = "SeedBag_Mushroom")]
    #[strum(
        props(
            name = "Mushroom Seeds",
            desc = "Grow a <link=ThingItemMushroom><color=green>Mushroom</color></link>.",
            value = "311593418"
        )
    )]
    SeedBagMushroom = 311593418i32,
    #[strum(serialize = "StructureCableCorner3Burnt")]
    #[strum(props(name = "Burnt Cable (3-Way Corner)", desc = "", value = "318437449"))]
    StructureCableCorner3Burnt = 318437449i32,
    #[strum(serialize = "StructureLogicSwitch2")]
    #[strum(props(name = "Switch", desc = "", value = "321604921"))]
    StructureLogicSwitch2 = 321604921i32,
    #[strum(serialize = "StructureOccupancySensor")]
    #[strum(
        props(
            name = "Occupancy Sensor",
            desc = "Will be triggered if there is a player in the same room as the sensor. The quantity variable will show the number of players. You can use configure it to only detect players who hold the correct Access Card using a <link=ThingCartridgeAccessController><color=green>Cartridge (Access Controller)</color></link> in a <link=ThingItemTablet><color=green>Handheld Tablet</color></link>. This sensor only works when placed in a room.",
            value = "322782515"
        )
    )]
    StructureOccupancySensor = 322782515i32,
    #[strum(serialize = "ItemKitSDBHopper")]
    #[strum(props(name = "Kit (SDB Hopper)", desc = "", value = "323957548"))]
    ItemKitSdbHopper = 323957548i32,
    #[strum(serialize = "ItemMKIIDrill")]
    #[strum(
        props(
            name = "Mk II Drill",
            desc = "The <link=ExMin><color=#0080FFFF>ExMin</color></link> Off-whirled Hand Drill has been a companion to <link=Stationeers><color=#0080FFFF>Stationeers</color></link> for decades. Essential for assembling and deconstructing various items and structures, regardless of gravity, pressure or temperature.",
            value = "324791548"
        )
    )]
    ItemMkiiDrill = 324791548i32,
    #[strum(serialize = "StructureCompositeFloorGrating")]
    #[strum(
        props(
            name = "Composite Floor Grating",
            desc = "While aesthetics rank low on the ladder of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> concerns, composite gratings allow the concealment of unsightly cables on floors, walls and ceilings.",
            value = "324868581"
        )
    )]
    StructureCompositeFloorGrating = 324868581i32,
    #[strum(serialize = "ItemKitSleeper")]
    #[strum(props(name = "Kit (Sleeper)", desc = "", value = "326752036"))]
    ItemKitSleeper = 326752036i32,
    #[strum(serialize = "EntityChickenBrown")]
    #[strum(
        props(
            name = "Entity Chicken Brown",
            desc = "Like so many of its brethren, this is a chicken. A brown one. It will eat soybeans, corn, and wheat, and lay eggs. Some will be fertilized, producing further chickens. Some will not.",
            value = "334097180"
        )
    )]
    EntityChickenBrown = 334097180i32,
    #[strum(serialize = "StructurePassiveVent")]
    #[strum(
        props(
            name = "Passive Vent",
            desc = "Passive vents allow gases to move into and out of pipe networks, which are closed systems unless connected to a device or structure. Passive vents are not powered, merely an aperture, essentially turning an enclosed space into part of the pipe network. ",
            value = "335498166"
        )
    )]
    StructurePassiveVent = 335498166i32,
    #[strum(serialize = "StructureAutolathe")]
    #[strum(
        props(
            name = "Autolathe",
            desc = "The foundation of most <link=Stationeers><color=#0080FFFF>Stationeer</color></link> fabrication systems, the <link=ExMin><color=#0080FFFF>ExMin</color></link> autolathe is a multi-axis molecular compositional system. Its complexity demands considerable time to assemble, but it remains an indispensable creation tool. Upgrade the device using a <link=ThingAutolathePrinterMod><color=green>Autolathe Printer Mod</color></link> for additional recipes and faster processing speeds.\n\t  ",
            value = "336213101"
        )
    )]
    StructureAutolathe = 336213101i32,
    #[strum(serialize = "AccessCardKhaki")]
    #[strum(props(name = "Access Card (Khaki)", desc = "", value = "337035771"))]
    AccessCardKhaki = 337035771i32,
    #[strum(serialize = "StructureBlastDoor")]
    #[strum(
        props(
            name = "Blast Door",
            desc = "Airtight and almost undamageable, the original 'Millmar' series of blast door was designed by off-world mining giant <link=Recurso><color=#0080FFFF>Recurso</color></link> to protect asteroid-mining facilities from nuclear-incident-level explosive decompression.\nShort of a pocket-sized singularity blinking into the local space-time frame, there is effectively no limit to the pressure these blast doors can contain - ideal for constructing airlocks in pressure-sensitive environments.",
            value = "337416191"
        )
    )]
    StructureBlastDoor = 337416191i32,
    #[strum(serialize = "ItemKitWeatherStation")]
    #[strum(props(name = "Kit (Weather Station)", desc = "", value = "337505889"))]
    ItemKitWeatherStation = 337505889i32,
    #[strum(serialize = "StructureStairwellFrontRight")]
    #[strum(props(name = "Stairwell (Front Right)", desc = "", value = "340210934"))]
    StructureStairwellFrontRight = 340210934i32,
    #[strum(serialize = "ItemKitGrowLight")]
    #[strum(props(name = "Kit (Grow Light)", desc = "", value = "341030083"))]
    ItemKitGrowLight = 341030083i32,
    #[strum(serialize = "StructurePictureFrameThickMountLandscapeSmall")]
    #[strum(
        props(
            name = "Picture Frame Thick Landscape Small",
            desc = "",
            value = "347154462"
        )
    )]
    StructurePictureFrameThickMountLandscapeSmall = 347154462i32,
    #[strum(serialize = "RoverCargo")]
    #[strum(
        props(
            name = "Rover (Cargo)",
            desc = "Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "350726273"
        )
    )]
    RoverCargo = 350726273i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidCrossJunction4")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (4-Way Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "363303270"
        )
    )]
    StructureInsulatedPipeLiquidCrossJunction4 = 363303270i32,
    #[strum(serialize = "ItemHardBackpack")]
    #[strum(
        props(
            name = "Hardsuit Backpack",
            desc = "This backpack can be useful when you are working inside and don't need to fly around.",
            value = "374891127"
        )
    )]
    ItemHardBackpack = 374891127i32,
    #[strum(serialize = "ItemKitDynamicLiquidCanister")]
    #[strum(props(name = "Kit (Portable Liquid Tank)", desc = "", value = "375541286"))]
    ItemKitDynamicLiquidCanister = 375541286i32,
    #[strum(serialize = "ItemKitGasGenerator")]
    #[strum(props(name = "Kit (Gas Fuel Generator)", desc = "", value = "377745425"))]
    ItemKitGasGenerator = 377745425i32,
    #[strum(serialize = "StructureBlocker")]
    #[strum(props(name = "Blocker", desc = "", value = "378084505"))]
    StructureBlocker = 378084505i32,
    #[strum(serialize = "StructurePressureFedLiquidEngine")]
    #[strum(
        props(
            name = "Pressure Fed Liquid Engine",
            desc = "Highly efficient and powerful, the Pressure Fed Liquid Engine is a challenging engine to run in a stable configuration. Liquid is pulled from the input into the engine based on the input gas pressure. Some gas is also moved in this process so Stationeers will need to devise a system to maintain a high gas pressure in the liquid input pipe. The second liquid pipe connection is an optional heat-exchanger connection which exchanges heat between the pipes contents and the engine bell, the Setting variable drives the effectiveness of the heat-exchanger.",
            value = "379750958"
        )
    )]
    StructurePressureFedLiquidEngine = 379750958i32,
    #[strum(serialize = "ItemPureIceNitrous")]
    #[strum(
        props(
            name = "Pure Ice NitrousOxide",
            desc = "A frozen chunk of pure <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link>",
            value = "386754635"
        )
    )]
    ItemPureIceNitrous = 386754635i32,
    #[strum(serialize = "StructureWallSmallPanelsMonoChrome")]
    #[strum(
        props(name = "Wall (Small Panels Mono Chrome)", desc = "", value = "386820253")
    )]
    StructureWallSmallPanelsMonoChrome = 386820253i32,
    #[strum(serialize = "ItemMKIIDuctTape")]
    #[strum(
        props(
            name = "Mk II Duct Tape",
            desc = "In the distant past, one of Earth's great champions taught a generation of 'Fix-It People' that duct tape was the answer to any problem. <link=Stationeers><color=#0080FFFF>Stationeers</color></link> have demonstrated that this is truth holds strong, so long as the problem is a damaged <link=ThingItemEvaSuit><color=green>Eva Suit</color></link>, <link=ThingItemJetpackBasic><color=green>Jetpack Basic</color></link>, <link=ThingItemSpaceHelmet><color=green>Space Helmet</color></link>, or even a <link=ThingStructureSolarPanel><color=green>Solar Panel</color></link>.\nTo use on yourself: put duct tape in your active hand, hold RIGHT MOUSE BUTTON to automatically repair damage.",
            value = "388774906"
        )
    )]
    ItemMkiiDuctTape = 388774906i32,
    #[strum(serialize = "ItemWreckageStructureRTG1")]
    #[strum(props(name = "Wreckage Structure RTG", desc = "", value = "391453348"))]
    ItemWreckageStructureRtg1 = 391453348i32,
    #[strum(serialize = "ItemPipeLabel")]
    #[strum(
        props(
            name = "Kit (Pipe Label)",
            desc = "This kit creates a <link=ThingStructurePipeLabel><color=green>Pipe Label</color></link>.",
            value = "391769637"
        )
    )]
    ItemPipeLabel = 391769637i32,
    #[strum(serialize = "DynamicGasCanisterPollutants")]
    #[strum(
        props(name = "Portable Gas Tank (Pollutants)", desc = "", value = "396065382")
    )]
    DynamicGasCanisterPollutants = 396065382i32,
    #[strum(serialize = "NpcChicken")]
    #[strum(props(name = "Chicken", desc = "", value = "399074198"))]
    NpcChicken = 399074198i32,
    #[strum(serialize = "RailingElegant01")]
    #[strum(props(name = "Railing Elegant (Type 1)", desc = "", value = "399661231"))]
    RailingElegant01 = 399661231i32,
    #[strum(serialize = "StructureBench1")]
    #[strum(props(name = "Bench (Counter Style)", desc = "", value = "406745009"))]
    StructureBench1 = 406745009i32,
    #[strum(serialize = "ItemAstroloyIngot")]
    #[strum(
        props(
            name = "Ingot (Astroloy)",
            desc = "Due to the original Stationeer manual collapsing into a singularity, Astroloy recipes have been warped by spacetime contortions. The correct Astroloy recipe, as memorialized for all time in a series of charming plastic icons, is 1.0 Copper, 1.0 Cobalt, and 2.0 Steel.",
            value = "412924554"
        )
    )]
    ItemAstroloyIngot = 412924554i32,
    #[strum(serialize = "ItemGasFilterCarbonDioxideM")]
    #[strum(
        props(name = "Medium Filter (Carbon Dioxide)", desc = "", value = "416897318")
    )]
    ItemGasFilterCarbonDioxideM = 416897318i32,
    #[strum(serialize = "ItemPillStun")]
    #[strum(
        props(
            name = "Pill (Paralysis)",
            desc = "Through rarely publicized, the existence of this pill is an open secret. For use when all else has failed, the Sayonara Suppository immobilizes and rapidly ends the average <link=Stationeers><color=#0080FFFF>Stationeer</color></link>. The delivery mode ensures that if a Stationeer chooses to take this pill, they really have to want it.",
            value = "418958601"
        )
    )]
    ItemPillStun = 418958601i32,
    #[strum(serialize = "ItemKitCrate")]
    #[strum(props(name = "Kit (Crate)", desc = "", value = "429365598"))]
    ItemKitCrate = 429365598i32,
    #[strum(serialize = "AccessCardPink")]
    #[strum(props(name = "Access Card (Pink)", desc = "", value = "431317557"))]
    AccessCardPink = 431317557i32,
    #[strum(serialize = "StructureWaterPipeMeter")]
    #[strum(props(name = "Liquid Pipe Meter", desc = "", value = "433184168"))]
    StructureWaterPipeMeter = 433184168i32,
    #[strum(serialize = "Robot")]
    #[strum(
        props(
            name = "AIMeE Bot",
            desc = "Designed by - presumably drunk - <link=Norsec><color=#0080FFFF>Norsec</color></link> roboticists, AIMeE (or Automated Independent Mechanical Entity) can be a <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> best friend, or tiresome nemesis, or both several times in the same day. \n      \nIntended to unearth and retrieve ores automatically, the unit requires basic programming knowledge to operate, and <link=ThingMotherboardProgrammableChip><color=green>IC Editor Motherboard</color></link>.\n\nAIMEe has 7 modes:\n\nRobotMode.None = 0 = Do nothing\nRobotMode.None = 1 = Follow nearest player\nRobotMode.None = 2 = Move to target in straight line\nRobotMode.None = 3 = Wander around looking for ores in 15 co-ords radius\nRobotMode.None = 4 = Unload in chute input or chute bin within 3 meters / 1.5 large grids\nRobotMode.None = 5 = Path(find) to target\nRobotMode.None = 6 = Automatic assigned state, shows when storage slots are fullConnects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "434786784"
        )
    )]
    Robot = 434786784i32,
    #[strum(serialize = "StructureChuteValve")]
    #[strum(
        props(
            name = "Chute Valve",
            desc = "The Chute Valve will stop the flow of materials when set to closed and when set to open, will act like a straight chute.",
            value = "434875271"
        )
    )]
    StructureChuteValve = 434875271i32,
    #[strum(serialize = "StructurePipeAnalysizer")]
    #[strum(
        props(
            name = "Pipe Analyzer",
            desc = "Allegedly the outcome of a weekend father-daughter electronics project by an overzealous {<link=ExMin><color=#0080FFFF>ExMin</color></link> engineer, the pipe analyzer is essentially a more advanced version of the <link=ThingStructurePipeMeter><color=green>Pipe Meter</color></link>.\nDisplaying the internal pressure of pipe networks, it  also reads out temperature and gas contents, and can be connected to a <link=ThingStructureConsole><color=green>Console</color></link> or <link=ThingStructureComputer><color=green>Computer</color></link> via a {<link=LogicPage><color=#0080FFFF>Logic</color></link> system.",
            value = "435685051"
        )
    )]
    StructurePipeAnalysizer = 435685051i32,
    #[strum(serialize = "StructureLogicBatchSlotReader")]
    #[strum(props(name = "Batch Slot Reader", desc = "", value = "436888930"))]
    StructureLogicBatchSlotReader = 436888930i32,
    #[strum(serialize = "StructureSatelliteDish")]
    #[strum(
        props(
            name = "Medium Satellite Dish",
            desc = "This medium communications unit can be used to communicate with nearby trade vessels.\n      \nWhen connected to a <link=ThingStructureComputer><color=green>Computer</color></link> containing a <link=ThingMotherboardComms><color=green>Communications Motherboard</color></link> motherboard, a <link=ThingLandingpad_CenterPiece01><color=green>Landingpad Center</color></link>, and a <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link>, this allows Stationeers to contact traders. Adjust its horizontal and vertical attributes either directly or through logic.",
            value = "439026183"
        )
    )]
    StructureSatelliteDish = 439026183i32,
    #[strum(serialize = "StructureIceCrusher")]
    #[strum(
        props(
            name = "Ice Crusher",
            desc = "The <link=Recurso><color=#0080FFFF>Recurso</color></link> KoolAuger converts various ices into their respective <link=GasPage><color=#0080FFFF>gases</color></link> and liquids.\nA remarkably smart and compact sublimation-melting unit, it produces gas or liquid depending on the ice being processed. The upper outlet is gas, the lower for liquid, and while you can attach any pipe you like to either outlet, it will only function if the correct network is attached. It will also only pass gas or liquid into a network if it is powered and turned on.\nIf the KoolAuger is full, it will not accept any further ice until the gas or liquid contents is drained. In this state, it will flash a yellow error state on the activation switch.",
            value = "443849486"
        )
    )]
    StructureIceCrusher = 443849486i32,
    #[strum(serialize = "PipeBenderMod")]
    #[strum(
        props(
            name = "Pipe Bender Mod",
            desc = "Apply to an <link=ThingStructureHydraulicPipeBender><color=green>Hydraulic Pipe Bender</color></link> with a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> or <link=ThingItemArcWelder><color=green>Arc Welder</color></link> to upgrade for increased processing speed and more recipe options.",
            value = "443947415"
        )
    )]
    PipeBenderMod = 443947415i32,
    #[strum(serialize = "StructureAdvancedComposter")]
    #[strum(
        props(
            name = "Advanced Composter",
            desc = "The advanced composter creates <link=ThingFertilizer><color=green>Fertilizer</color></link> out of organic matter. It accepts <link=OrganicPage><color=#0080FFFF>food</color></link>, <link=ThingDecayedFood><color=green>Decayed Food</color></link> or <link=ThingItemBiomass><color=green>Biomass</color></link>. It requires <link=GasWater><color=#44AD83>Water</color></link> and power to operate, accelerating the natural composting process.\nWhen processing, it releases nitrogen and volatiles, as well a small amount of heat. \n\n<size=120%><b>Compost composition</b></size>\nFertilizer is produced at a 1:3 ratio of fertilizer to ingredients. The fertilizer's effects on plants will vary depending on the respective proportions of its ingredients.\n\n- <link=OrganicPage><color=#0080FFFF>Food</color></link> increases PLANT YIELD up to two times\n- <link=ThingDecayedFood><color=green>Decayed Food</color></link> increases plant GROWTH SPEED up to two times\n- <link=ThingItemBiomass><color=green>Biomass</color></link> increases the NUMBER OF GROWTH CYCLES the fertilizer lasts for up to five times\n",
            value = "446212963"
        )
    )]
    StructureAdvancedComposter = 446212963i32,
    #[strum(serialize = "ItemKitLargeDirectHeatExchanger")]
    #[strum(
        props(name = "Kit (Large Direct Heat Exchanger)", desc = "", value = "450164077")
    )]
    ItemKitLargeDirectHeatExchanger = 450164077i32,
    #[strum(serialize = "ItemKitInsulatedPipe")]
    #[strum(props(name = "Kit (Insulated Pipe)", desc = "", value = "452636699"))]
    ItemKitInsulatedPipe = 452636699i32,
    #[strum(serialize = "ItemCocoaPowder")]
    #[strum(props(name = "Cocoa Powder", desc = "", value = "457286516"))]
    ItemCocoaPowder = 457286516i32,
    #[strum(serialize = "AccessCardPurple")]
    #[strum(props(name = "Access Card (Purple)", desc = "", value = "459843265"))]
    AccessCardPurple = 459843265i32,
    #[strum(serialize = "ItemGasFilterNitrousOxideL")]
    #[strum(
        props(name = "Heavy Filter (Nitrous Oxide)", desc = "", value = "465267979")
    )]
    ItemGasFilterNitrousOxideL = 465267979i32,
    #[strum(serialize = "StructurePipeCowl")]
    #[strum(props(name = "Pipe Cowl", desc = "", value = "465816159"))]
    StructurePipeCowl = 465816159i32,
    #[strum(serialize = "StructureSDBHopperAdvanced")]
    #[strum(props(name = "SDB Hopper Advanced", desc = "", value = "467225612"))]
    StructureSdbHopperAdvanced = 467225612i32,
    #[strum(serialize = "StructureCableJunctionH")]
    #[strum(
        props(name = "Heavy Cable (3-Way Junction)", desc = "", value = "469451637")
    )]
    StructureCableJunctionH = 469451637i32,
    #[strum(serialize = "ItemHEMDroidRepairKit")]
    #[strum(
        props(
            name = "HEMDroid Repair Kit",
            desc = "Repairs damaged HEM-Droids to full health.",
            value = "470636008"
        )
    )]
    ItemHemDroidRepairKit = 470636008i32,
    #[strum(serialize = "ItemKitRocketCargoStorage")]
    #[strum(props(name = "Kit (Rocket Cargo Storage)", desc = "", value = "479850239"))]
    ItemKitRocketCargoStorage = 479850239i32,
    #[strum(serialize = "StructureLiquidPressureRegulator")]
    #[strum(
        props(
            name = "Liquid Volume Regulator",
            desc = "Regulates the volume ratio of liquid in the output Liquid pipe. This is expressed as percentage where 100 is totally full and 0 is empty.",
            value = "482248766"
        )
    )]
    StructureLiquidPressureRegulator = 482248766i32,
    #[strum(serialize = "SeedBag_Switchgrass")]
    #[strum(props(name = "Switchgrass Seed", desc = "", value = "488360169"))]
    SeedBagSwitchgrass = 488360169i32,
    #[strum(serialize = "ItemKitLadder")]
    #[strum(props(name = "Kit (Ladder)", desc = "", value = "489494578"))]
    ItemKitLadder = 489494578i32,
    #[strum(serialize = "StructureLogicButton")]
    #[strum(props(name = "Button", desc = "", value = "491845673"))]
    StructureLogicButton = 491845673i32,
    #[strum(serialize = "ItemRTG")]
    #[strum(
        props(
            name = "Kit (Creative RTG)",
            desc = "This kit creates that miracle of modern science, a <link=ThingItemRTG><color=green>Kit (Creative RTG)</color></link>.",
            value = "495305053"
        )
    )]
    ItemRtg = 495305053i32,
    #[strum(serialize = "ItemKitAIMeE")]
    #[strum(props(name = "Kit (AIMeE)", desc = "", value = "496830914"))]
    ItemKitAiMeE = 496830914i32,
    #[strum(serialize = "ItemSprayCanWhite")]
    #[strum(
        props(
            name = "Spray Paint (White)",
            desc = "White looks clean, sharp and nice. But <link=Stationeers><color=#0080FFFF>Stationeering</color></link> can be a dirty job. White tends to scuff.",
            value = "498481505"
        )
    )]
    ItemSprayCanWhite = 498481505i32,
    #[strum(serialize = "ItemElectrumIngot")]
    #[strum(props(name = "Ingot (Electrum)", desc = "", value = "502280180"))]
    ItemElectrumIngot = 502280180i32,
    #[strum(serialize = "MotherboardLogic")]
    #[strum(
        props(
            name = "Logic Motherboard",
            desc = "Motherboards are connected to <link=ThingStructureComputer><color=green>Computer</color></link>s to perform various technical functions.\nThe <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> K-cops logic motherboard allows <link=Stationeers><color=#0080FFFF>Stationeers</color></link> to set variables and actions on specific <link=LogicUnitPage><color=#0080FFFF>logic</color></link>-controlled items.",
            value = "502555944"
        )
    )]
    MotherboardLogic = 502555944i32,
    #[strum(serialize = "StructureStairwellBackLeft")]
    #[strum(props(name = "Stairwell (Back Left)", desc = "", value = "505924160"))]
    StructureStairwellBackLeft = 505924160i32,
    #[strum(serialize = "ItemKitAccessBridge")]
    #[strum(props(name = "Kit (Access Bridge)", desc = "", value = "513258369"))]
    ItemKitAccessBridge = 513258369i32,
    #[strum(serialize = "StructureRocketTransformerSmall")]
    #[strum(props(name = "Transformer Small (Rocket)", desc = "", value = "518925193"))]
    StructureRocketTransformerSmall = 518925193i32,
    #[strum(serialize = "DynamicAirConditioner")]
    #[strum(
        props(
            name = "Portable Air Conditioner",
            desc = "The <link=Sinotai><color=#0080FFFF>Sinotai</color></link>-designed Huxi portable air conditioner cools by drawing heat from the atmosphere and storing it, or adding heat to the atmosphere from its internal tank. With a max internal pressure of 8106kPa, its capacity is relatively limited, physics being clear on this subject. To extend its temperature storage ability, bolt the Huxi to a <link=ThingStructureTankConnector><color=green>Tank Connector</color></link>, then connect it to a pipe network supplying hot or cold <link=GasPage><color=#0080FFFF>gases</color></link>.",
            value = "519913639"
        )
    )]
    DynamicAirConditioner = 519913639i32,
    #[strum(serialize = "ItemKitToolManufactory")]
    #[strum(props(name = "Kit (Tool Manufactory)", desc = "", value = "529137748"))]
    ItemKitToolManufactory = 529137748i32,
    #[strum(serialize = "ItemKitSign")]
    #[strum(props(name = "Kit (Sign)", desc = "", value = "529996327"))]
    ItemKitSign = 529996327i32,
    #[strum(serialize = "StructureCompositeCladdingSphericalCap")]
    #[strum(
        props(
            name = "Composite Cladding (Spherical Cap)",
            desc = "",
            value = "534213209"
        )
    )]
    StructureCompositeCladdingSphericalCap = 534213209i32,
    #[strum(serialize = "ItemPureIceLiquidOxygen")]
    #[strum(
        props(
            name = "Pure Ice Liquid Oxygen",
            desc = "A frozen chunk of pure <link=GasLiquidOxygen><color=#44AD83>Liquid Oxygen</color></link>",
            value = "541621589"
        )
    )]
    ItemPureIceLiquidOxygen = 541621589i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation003")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "542009679"
        )
    )]
    ItemWreckageStructureWeatherStation003 = 542009679i32,
    #[strum(serialize = "StructureInLineTankLiquid1x1")]
    #[strum(
        props(
            name = "In-Line Tank Small Liquid",
            desc = "A small expansion tank that increases the volume of a pipe network.",
            value = "543645499"
        )
    )]
    StructureInLineTankLiquid1X1 = 543645499i32,
    #[strum(serialize = "ItemBatteryCellNuclear")]
    #[strum(
        props(
            name = "Battery Cell (Nuclear)",
            desc = "Illegal on Earth since the Chengdu Event, Norsec nuclear power cells found a new and drastically less safety-conscious market offworld.\n\n<size=120%><b>POWER OUTPUT</b></size>\nPushing the power-weight balance to its limits, the 'nuke' has a 2.3 megawatt charge (2304000W), reflecting its reliance on exotic <link=IngotPage><color=#0080FFFF>superalloys</color></link>.",
            value = "544617306"
        )
    )]
    ItemBatteryCellNuclear = 544617306i32,
    #[strum(serialize = "ItemCornSoup")]
    #[strum(
        props(
            name = "Corn Soup",
            desc = "Made using <link=ThingItemCookedCorn><color=green>Cooked Corn</color></link> and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link> in a <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link> or <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link>. Faily high in nutrition, canned food does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "545034114"
        )
    )]
    ItemCornSoup = 545034114i32,
    #[strum(serialize = "StructureAdvancedFurnace")]
    #[strum(
        props(
            name = "Advanced Furnace",
            desc = "The advanced furnace comes with integrated inlet and outlet pumps for controlling the unit's internal pressure.",
            value = "545937711"
        )
    )]
    StructureAdvancedFurnace = 545937711i32,
    #[strum(serialize = "StructureLogicRocketUplink")]
    #[strum(props(name = "Logic Uplink", desc = "", value = "546002924"))]
    StructureLogicRocketUplink = 546002924i32,
    #[strum(serialize = "StructureLogicDial")]
    #[strum(
        props(
            name = "Dial",
            desc = "An assignable dial with up to 1000 modes.",
            value = "554524804"
        )
    )]
    StructureLogicDial = 554524804i32,
    #[strum(serialize = "StructureLightLongWide")]
    #[strum(props(name = "Wall Light (Long Wide)", desc = "", value = "555215790"))]
    StructureLightLongWide = 555215790i32,
    #[strum(serialize = "StructureProximitySensor")]
    #[strum(
        props(
            name = "Proximity Sensor",
            desc = "Will be triggered if there is a player in the range of the sensor (as defined by the setting dial). The quantity variable will show the number of players. You can configure the sensor to only detect players who hold the correct Access Card using a <link=ThingCartridgeAccessController><color=green>Cartridge (Access Controller)</color></link> in a <link=ThingItemTablet><color=green>Handheld Tablet</color></link>.",
            value = "568800213"
        )
    )]
    StructureProximitySensor = 568800213i32,
    #[strum(serialize = "AccessCardYellow")]
    #[strum(props(name = "Access Card (Yellow)", desc = "", value = "568932536"))]
    AccessCardYellow = 568932536i32,
    #[strum(serialize = "StructureDiodeSlide")]
    #[strum(props(name = "Diode Slide", desc = "", value = "576516101"))]
    StructureDiodeSlide = 576516101i32,
    #[strum(serialize = "ItemKitSecurityPrinter")]
    #[strum(props(name = "Kit (Security Printer)", desc = "", value = "578078533"))]
    ItemKitSecurityPrinter = 578078533i32,
    #[strum(serialize = "ItemKitCentrifuge")]
    #[strum(props(name = "Kit (Centrifuge)", desc = "", value = "578182956"))]
    ItemKitCentrifuge = 578182956i32,
    #[strum(serialize = "DynamicHydroponics")]
    #[strum(props(name = "Portable Hydroponics", desc = "", value = "587726607"))]
    DynamicHydroponics = 587726607i32,
    #[strum(serialize = "ItemKitPipeUtilityLiquid")]
    #[strum(props(name = "Kit (Pipe Utility Liquid)", desc = "", value = "595478589"))]
    ItemKitPipeUtilityLiquid = 595478589i32,
    #[strum(serialize = "StructureCompositeFloorGrating4")]
    #[strum(
        props(name = "Composite Floor Grating  (Type 4)", desc = "", value = "600133846")
    )]
    StructureCompositeFloorGrating4 = 600133846i32,
    #[strum(serialize = "StructureCableStraight")]
    #[strum(
        props(
            name = "Cable (Straight)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life - so much so, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link>.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "605357050"
        )
    )]
    StructureCableStraight = 605357050i32,
    #[strum(serialize = "StructureLiquidTankSmallInsulated")]
    #[strum(props(name = "Insulated Liquid Tank Small", desc = "", value = "608607718"))]
    StructureLiquidTankSmallInsulated = 608607718i32,
    #[strum(serialize = "ItemKitWaterPurifier")]
    #[strum(props(name = "Kit (Water Purifier)", desc = "", value = "611181283"))]
    ItemKitWaterPurifier = 611181283i32,
    #[strum(serialize = "ItemKitLiquidTankInsulated")]
    #[strum(props(name = "Kit (Insulated Liquid Tank)", desc = "", value = "617773453"))]
    ItemKitLiquidTankInsulated = 617773453i32,
    #[strum(serialize = "StructureWallSmallPanelsAndHatch")]
    #[strum(
        props(name = "Wall (Small Panels And Hatch)", desc = "", value = "619828719")
    )]
    StructureWallSmallPanelsAndHatch = 619828719i32,
    #[strum(serialize = "ItemGasFilterNitrogen")]
    #[strum(
        props(
            name = "Filter (Nitrogen)",
            desc = "Filters are used to capture various gases, which can be disposed of or used elsewhere. <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> is a byproduct of smelting various ores, notably <link=ThingItemNitrice><color=green>Ice (Nitrice)</color></link>, which may be combined with <link=GasOxygen><color=#44AD83>Oxygen</color></link> to make a breathable - and considerably less flammable - atmosphere.",
            value = "632853248"
        )
    )]
    ItemGasFilterNitrogen = 632853248i32,
    #[strum(serialize = "ReagentColorYellow")]
    #[strum(props(name = "Color Dye (Yellow)", desc = "", value = "635208006"))]
    ReagentColorYellow = 635208006i32,
    #[strum(serialize = "StructureWallPadding")]
    #[strum(props(name = "Wall (Padding)", desc = "", value = "635995024"))]
    StructureWallPadding = 635995024i32,
    #[strum(serialize = "ItemKitPassthroughHeatExchanger")]
    #[strum(
        props(name = "Kit (CounterFlow Heat Exchanger)", desc = "", value = "636112787")
    )]
    ItemKitPassthroughHeatExchanger = 636112787i32,
    #[strum(serialize = "StructureChuteDigitalValveLeft")]
    #[strum(
        props(
            name = "Chute Digital Valve Left",
            desc = "The Digital Chute Valve will stop the flow of materials when set to closed and when set to open, will act like a straight chute. The valve will automatically close after a certain number of items have passed through. This threshold can be set using the dial.",
            value = "648608238"
        )
    )]
    StructureChuteDigitalValveLeft = 648608238i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadHighSpeedIce")]
    #[strum(
        props(
            name = "Mining-Drill Head (High Speed Ice)",
            desc = "",
            value = "653461728"
        )
    )]
    ItemRocketMiningDrillHeadHighSpeedIce = 653461728i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation007")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "656649558"
        )
    )]
    ItemWreckageStructureWeatherStation007 = 656649558i32,
    #[strum(serialize = "ItemRice")]
    #[strum(
        props(
            name = "Rice",
            desc = "Rice grows at a moderate rate as long as its supplied with plenty of water.  Being more dependant on water, rice plants can easily die during periods of drought.",
            value = "658916791"
        )
    )]
    ItemRice = 658916791i32,
    #[strum(serialize = "ItemPlasticSheets")]
    #[strum(props(name = "Plastic Sheets", desc = "", value = "662053345"))]
    ItemPlasticSheets = 662053345i32,
    #[strum(serialize = "ItemKitTransformerSmall")]
    #[strum(props(name = "Kit (Transformer Small)", desc = "", value = "665194284"))]
    ItemKitTransformerSmall = 665194284i32,
    #[strum(serialize = "StructurePipeLiquidStraight")]
    #[strum(
        props(
            name = "Liquid Pipe (Straight)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidStraight><color=green>Insulated Liquid Pipe (Straight)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "667597982"
        )
    )]
    StructurePipeLiquidStraight = 667597982i32,
    #[strum(serialize = "ItemSpaceIce")]
    #[strum(props(name = "Space Ice", desc = "", value = "675686937"))]
    ItemSpaceIce = 675686937i32,
    #[strum(serialize = "ItemRemoteDetonator")]
    #[strum(props(name = "Remote Detonator", desc = "", value = "678483886"))]
    ItemRemoteDetonator = 678483886i32,
    #[strum(serialize = "ItemCocoaTree")]
    #[strum(props(name = "Cocoa", desc = "", value = "680051921"))]
    ItemCocoaTree = 680051921i32,
    #[strum(serialize = "ItemKitAirlockGate")]
    #[strum(props(name = "Kit (Hangar Door)", desc = "", value = "682546947"))]
    ItemKitAirlockGate = 682546947i32,
    #[strum(serialize = "ItemScrewdriver")]
    #[strum(
        props(
            name = "Screwdriver",
            desc = "This standard issue frictional adherence adjustor is a top of the line, bi-rotational model with a columnated uni-grip. It's definitely not just a screwdriver. Use it for construction and deconstruction of certain kits, and setting values on <link=LogicUnitPage><color=#0080FFFF>logic</color></link> units.",
            value = "687940869"
        )
    )]
    ItemScrewdriver = 687940869i32,
    #[strum(serialize = "ItemTomatoSoup")]
    #[strum(
        props(
            name = "Tomato Soup",
            desc = "Made using <link=ThingItemCookedTomato><color=green>Cooked Tomato</color></link>s and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link> in a <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link> or <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link>.",
            value = "688734890"
        )
    )]
    ItemTomatoSoup = 688734890i32,
    #[strum(serialize = "StructureCentrifuge")]
    #[strum(
        props(
            name = "Centrifuge",
            desc = "If a <link=ThingStructureRecycler><color=green>Recycler</color></link> or unbalanced <link=ThingStructureFurnace><color=green>Furnace</color></link> outputs <link=ReagentPage><color=#0080FFFF>reagent</color></link> mixture rather than the desired <link=IngotPage><color=#0080FFFF>ingots</color></link>, a centrifuge allows you to reclaim the raw <link=OrePage><color=#0080FFFF>ore</color></link>. \n        It also refines <link=ThingItemDirtyOre><color=green>Dirty Ore</color></link> produced from the <link=ThingStructureDeepMiner><color=green>Deep Miner</color></link> and <link=ThingItemSpaceOre><color=green>Dirty Ore</color></link> produced from the <link=ThingStructureRocketMiner><color=green>Rocket Miner</color></link>. \n        Its bigger brother <link=ThingStructureCombustionCentrifuge><color=green>Combustion Centrifuge</color></link> can be used to process items significantly faster. Items processed by the centrifuge will be de-gassed. \n        If openned while powered on, the centrifuge will enter an errored state and reduce its rpm to 0 and then export any items.",
            value = "690945935"
        )
    )]
    StructureCentrifuge = 690945935i32,
    #[strum(serialize = "StructureBlockBed")]
    #[strum(
        props(name = "Block Bed", desc = "Description coming.", value = "697908419")
    )]
    StructureBlockBed = 697908419i32,
    #[strum(serialize = "ItemBatteryCell")]
    #[strum(
        props(
            name = "Battery Cell (Small)",
            desc = "Harnessing a design pioneered in the early 21st century, the small battery cell is the <link=Stationeers><color=#0080FFFF>Stationeer's</color></link> basic unit of portable electrical power. While it lacks the charge of a <link=ThingItemBatteryCellLarge><color=green>Battery Cell (Large)</color></link> or <link=ThingItemBatteryCellNuclear><color=green>Battery Cell (Nuclear)</color></link>, it has the humble advantage of being fabricated from basic resources.\n\n<size=120%><b>POWER OUTPUT</b></size>\nThe small cell stores up to 36000 watts of power.",
            value = "700133157"
        )
    )]
    ItemBatteryCell = 700133157i32,
    #[strum(serialize = "ItemSpaceHelmet")]
    #[strum(
        props(
            name = "Space Helmet",
            desc = "The basic space helmet insulates <link=Stationeers><color=#0080FFFF>Stationeers</color></link> against everything from hard vacuum to weird cooking smells. Providing a pressure-controlled, breathable atmosphere, it comes with a built-in light powered by your <link=ThingItemEvaSuit><color=green>Eva Suit</color></link> <link=ThingItemBatteryCell><color=green>Battery Cell (Small)</color></link>.\nIt also incorporates a lock/unlock feature to avoid accidental opening, as well as a flush function to expel and replace the internal atmosphere. If damaged, use <link=ThingItemDuctTape><color=green>Duct Tape</color></link> to fix it, or paint it any color you like using the <link=ThingAppliancePaintMixer><color=green>Paint Mixer</color></link>.",
            value = "714830451"
        )
    )]
    ItemSpaceHelmet = 714830451i32,
    #[strum(serialize = "StructureCompositeWall02")]
    #[strum(props(name = "Composite Wall (Type 2)", desc = "", value = "718343384"))]
    StructureCompositeWall02 = 718343384i32,
    #[strum(serialize = "ItemKitRocketCircuitHousing")]
    #[strum(
        props(name = "Kit (Rocket Circuit Housing)", desc = "", value = "721251202")
    )]
    ItemKitRocketCircuitHousing = 721251202i32,
    #[strum(serialize = "ItemKitResearchMachine")]
    #[strum(props(name = "Kit Research Machine", desc = "", value = "724776762"))]
    ItemKitResearchMachine = 724776762i32,
    #[strum(serialize = "ItemElectronicParts")]
    #[strum(props(name = "Electronic Parts", desc = "", value = "731250882"))]
    ItemElectronicParts = 731250882i32,
    #[strum(serialize = "ItemKitShower")]
    #[strum(props(name = "Kit (Shower)", desc = "", value = "735858725"))]
    ItemKitShower = 735858725i32,
    #[strum(serialize = "StructureUnloader")]
    #[strum(
        props(
            name = "Unloader",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> Re:Gurge is a handy unit for unloading any items inserted into it, and feeding them into a chute network. For instance, if you add a full <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>, the Re:Gurge will empty a mining belt of its contents, insert them into the chute network, then insert the mining belt itself. A <link=ThingStructureSorter><color=green>Sorter</color></link> is recommended to reclaim the mining belt.\n\nOutput = 0 exporting the main item\nOutput = 1 exporting items inside and eventually the main item.",
            value = "750118160"
        )
    )]
    StructureUnloader = 750118160i32,
    #[strum(serialize = "ItemKitRailing")]
    #[strum(props(name = "Kit (Railing)", desc = "", value = "750176282"))]
    ItemKitRailing = 750176282i32,
    #[strum(serialize = "StructureFridgeSmall")]
    #[strum(
        props(
            name = "Fridge Small",
            desc = "Essentially a heavily insulated box that allows users to pipe in any desired atmosphere, the <link=Recurso><color=#0080FFFF>Recurso</color></link> Minibar fridge was a simple solution to the problem of <link=NutritionPage><color=#0080FFFF>food decay</color></link>. It stores a small number of items, at any temperature you can muster.\n      \n      For more information about food preservation, visit the <link=DecayPage><color=#0080FFFF>food decay</color></link> section of the Stationpedia.",
            value = "751887598"
        )
    )]
    StructureFridgeSmall = 751887598i32,
    #[strum(serialize = "DynamicScrubber")]
    #[strum(
        props(
            name = "Portable Air Scrubber",
            desc = "A portable scrubber does just what it sounds like: removes specific substances from the air. For instance, attaching a <link=ThingItemGasFilterCarbonDioxide><color=green>Filter (Carbon Dioxide)</color></link> will pull <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> from the surrounding atmosphere. Note that the scrubber has room for one battery and two filters, which will double its operating speed. Neat. When it reaches an internal pressure of 8106kPA, an error signal will flash on the switch, indicating it needs to be emptied. Either vent it directly, or attach it to a pipe network via a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "755048589"
        )
    )]
    DynamicScrubber = 755048589i32,
    #[strum(serialize = "ItemKitEngineLarge")]
    #[strum(props(name = "Kit (Engine Large)", desc = "", value = "755302726"))]
    ItemKitEngineLarge = 755302726i32,
    #[strum(serialize = "ItemKitTank")]
    #[strum(props(name = "Kit (Tank)", desc = "", value = "771439840"))]
    ItemKitTank = 771439840i32,
    #[strum(serialize = "ItemLiquidCanisterSmart")]
    #[strum(
        props(
            name = "Liquid Canister (Smart)",
            desc = "0.Mode0\n1.Mode1",
            value = "777684475"
        )
    )]
    ItemLiquidCanisterSmart = 777684475i32,
    #[strum(serialize = "StructureWallArchTwoTone")]
    #[strum(props(name = "Wall (Arch Two Tone)", desc = "", value = "782529714"))]
    StructureWallArchTwoTone = 782529714i32,
    #[strum(serialize = "ItemAuthoringTool")]
    #[strum(props(name = "Authoring Tool", desc = "", value = "789015045"))]
    ItemAuthoringTool = 789015045i32,
    #[strum(serialize = "WeaponEnergy")]
    #[strum(props(name = "Weapon Energy", desc = "", value = "789494694"))]
    WeaponEnergy = 789494694i32,
    #[strum(serialize = "ItemCerealBar")]
    #[strum(
        props(
            name = "Cereal Bar",
            desc = "Sustains, without decay. If only all our relationships were so well balanced.",
            value = "791746840"
        )
    )]
    ItemCerealBar = 791746840i32,
    #[strum(serialize = "StructureLargeDirectHeatExchangeLiquidtoLiquid")]
    #[strum(
        props(
            name = "Large Direct Heat Exchange - Liquid + Liquid",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "792686502"
        )
    )]
    StructureLargeDirectHeatExchangeLiquidtoLiquid = 792686502i32,
    #[strum(serialize = "StructureLightLong")]
    #[strum(props(name = "Wall Light (Long)", desc = "", value = "797794350"))]
    StructureLightLong = 797794350i32,
    #[strum(serialize = "StructureWallIron03")]
    #[strum(props(name = "Iron Wall (Type 3)", desc = "", value = "798439281"))]
    StructureWallIron03 = 798439281i32,
    #[strum(serialize = "ItemPipeValve")]
    #[strum(
        props(
            name = "Kit (Pipe Valve)",
            desc = "This kit creates a <link=ThingStructureValve><color=green>Valve</color></link>.",
            value = "799323450"
        )
    )]
    ItemPipeValve = 799323450i32,
    #[strum(serialize = "StructureConsoleMonitor")]
    #[strum(
        props(
            name = "Console Monitor",
            desc = "This <link=Norsec><color=#0080FFFF>Norsec-designed</color></link> control box manages devices such as the <link=ThingStructureActiveVent><color=green>Active Vent</color></link>, <link=ThingStructurePassiveVent><color=green>Passive Vent</color></link>, <link=ThingStructureGasSensor><color=green>Gas Sensor</color></link>, <link=ThingItemSecurityCamera><color=green>Security Camera</color></link> and <link=ThingStructureCompositeDoor><color=green>Composite Door</color></link>, depending on which <link=LogicPage><color=#0080FFFF>circuitboard</color></link> is inserted into the unit. It has a shared data/power port, and a charming sloped interface.\nA completed console displays all devices connected to the current power network. Any devices not related to the installed circuitboard will be greyed-out and inoperable. Consoles are locked once a <link=ThingItemDataDisk><color=green>Data Disk</color></link> is removed.",
            value = "801677497"
        )
    )]
    StructureConsoleMonitor = 801677497i32,
    #[strum(serialize = "StructureRover")]
    #[strum(props(name = "Rover Frame", desc = "", value = "806513938"))]
    StructureRover = 806513938i32,
    #[strum(serialize = "StructureRocketAvionics")]
    #[strum(props(name = "Rocket Avionics", desc = "", value = "808389066"))]
    StructureRocketAvionics = 808389066i32,
    #[strum(serialize = "UniformOrangeJumpSuit")]
    #[strum(props(name = "Jump Suit (Orange)", desc = "", value = "810053150"))]
    UniformOrangeJumpSuit = 810053150i32,
    #[strum(serialize = "StructureSolidFuelGenerator")]
    #[strum(
        props(
            name = "Generator (Solid Fuel)",
            desc = "The mainstay of power generation for Stationeers, this device provides 20kW of power. Multiple solid resources can be loaded. While operating, the device will output its maximum power regardless of whether you have captured it or not. Watch for blown wires! It will output much more power than your regular <link=ThingItemCableCoil><color=green>Cable Coil</color></link> can handle.",
            value = "813146305"
        )
    )]
    StructureSolidFuelGenerator = 813146305i32,
    #[strum(serialize = "Landingpad_GasConnectorInwardPiece")]
    #[strum(props(name = "Landingpad Gas Input", desc = "", value = "817945707"))]
    LandingpadGasConnectorInwardPiece = 817945707i32,
    #[strum(serialize = "StructureElevatorShaft")]
    #[strum(props(name = "Elevator Shaft (Cabled)", desc = "", value = "826144419"))]
    StructureElevatorShaft = 826144419i32,
    #[strum(serialize = "StructureTransformerMediumReversed")]
    #[strum(
        props(
            name = "Transformer Reversed (Medium)",
            desc = "Transformers control the maximum power that will flow down a sub-network of cables, to prevent overloading <link=ElectronicPage><color=#0080FFFF>electrical</color></link> systems. \nMedium transformers are used in larger setups where more than 5000W is required, with output that can be set to a maximum of 25000W.\nNote that transformers also operate as data isolators, preventing data flowing into any network beyond it.",
            value = "833912764"
        )
    )]
    StructureTransformerMediumReversed = 833912764i32,
    #[strum(serialize = "StructureFlatBench")]
    #[strum(props(name = "Bench (Flat)", desc = "", value = "839890807"))]
    StructureFlatBench = 839890807i32,
    #[strum(serialize = "ItemPowerConnector")]
    #[strum(
        props(
            name = "Kit (Power Connector)",
            desc = "This kit creates a <link=ThingStructurePowerConnector><color=green>Power Connector</color></link>.",
            value = "839924019"
        )
    )]
    ItemPowerConnector = 839924019i32,
    #[strum(serialize = "ItemKitHorizontalAutoMiner")]
    #[strum(props(name = "Kit (OGRE)", desc = "", value = "844391171"))]
    ItemKitHorizontalAutoMiner = 844391171i32,
    #[strum(serialize = "ItemKitSolarPanelBasic")]
    #[strum(props(name = "Kit (Solar Panel Basic)", desc = "", value = "844961456"))]
    ItemKitSolarPanelBasic = 844961456i32,
    #[strum(serialize = "ItemSprayCanBrown")]
    #[strum(
        props(
            name = "Spray Paint (Brown)",
            desc = "In more artistic <link=Stationeers><color=#0080FFFF>Stationeers</color></link> circles, the absence of brown is often lamented, but seldom changed.",
            value = "845176977"
        )
    )]
    ItemSprayCanBrown = 845176977i32,
    #[strum(serialize = "ItemKitLargeExtendableRadiator")]
    #[strum(
        props(name = "Kit (Large Extendable Radiator)", desc = "", value = "847430620")
    )]
    ItemKitLargeExtendableRadiator = 847430620i32,
    #[strum(serialize = "StructureInteriorDoorPadded")]
    #[strum(
        props(
            name = "Interior Door Padded",
            desc = "0.Operate\n1.Logic",
            value = "847461335"
        )
    )]
    StructureInteriorDoorPadded = 847461335i32,
    #[strum(serialize = "ItemKitRecycler")]
    #[strum(props(name = "Kit (Recycler)", desc = "", value = "849148192"))]
    ItemKitRecycler = 849148192i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerLong")]
    #[strum(
        props(
            name = "Composite Cladding (Long Angled Corner)",
            desc = "",
            value = "850558385"
        )
    )]
    StructureCompositeCladdingAngledCornerLong = 850558385i32,
    #[strum(serialize = "ItemPlantEndothermic_Genepool1")]
    #[strum(
        props(
            name = "Winterspawn (Alpha variant)",
            desc = "<link=Agrizero><color=#0080FFFF>Agrizero's</color></link> Winterspawn atmospheric bio-processor is a recent addition to their catalog of genespliced environmental decorations. Using ambient heat to split <link=GasWater><color=#44AD83>Water</color></link> into <link=GasVolatiles><color=#44AD83>Volatiles</color></link> and <link=GasOxygen><color=#44AD83>Oxygen</color></link>, the Winterspawn cools its surroundings, when supplied with sufficient <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>. The alpha variant has a peak cooling and electrolysis capacity of 90Watts and is most efficient operating in air temperatures of 0 to 40 Degrees Celsius.",
            value = "851290561"
        )
    )]
    ItemPlantEndothermicGenepool1 = 851290561i32,
    #[strum(serialize = "CircuitboardDoorControl")]
    #[strum(
        props(
            name = "Door Control",
            desc = "A basic tool of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> base construction, this circuit board provides a way to open and close a <link=ThingStructureCompositeDoor><color=green>Composite Door</color></link>, <link=ThingStructureBlastDoor><color=green>Blast Door</color></link> or <link=ThingStructureGlassDoor><color=green>Glass Door</color></link> remotely, when connected to a <link=ThingStructureConsole><color=green>Console</color></link>. This system can be further linked to <link=ThingStructureMotionSensor><color=green>Motion Sensor</color></link> to create automatic doors.",
            value = "855694771"
        )
    )]
    CircuitboardDoorControl = 855694771i32,
    #[strum(serialize = "ItemCrowbar")]
    #[strum(
        props(
            name = "Crowbar",
            desc = "<link=Recurso><color=#0080FFFF>Recurso's</color></link> entry-level crowbar is useful in a variety of everyday <link=Stationeers><color=#0080FFFF>Stationeer</color></link> settings, from opening <link=ThingStructureAreaPowerControl><color=green>Area Power Control</color></link>s and unpowered <link=ThingStructureAirlock><color=green>Airlock</color></link>s, to splatting pan-dimensional headcrabs, should the need arise.",
            value = "856108234"
        )
    )]
    ItemCrowbar = 856108234i32,
    #[strum(serialize = "ItemChocolateCerealBar")]
    #[strum(props(name = "Chocolate Cereal Bar", desc = "", value = "860793245"))]
    ItemChocolateCerealBar = 860793245i32,
    #[strum(serialize = "Rover_MkI_build_states")]
    #[strum(props(name = "Rover MKI", desc = "", value = "861674123"))]
    RoverMkIBuildStates = 861674123i32,
    #[strum(serialize = "AppliancePlantGeneticStabilizer")]
    #[strum(
        props(
            name = "Plant Genetic Stabilizer",
            desc = "The Genetic Stabilizer can be used to manipulate gene stability on a specific <link=OrganicPage><color=#0080FFFF>Plants</color></link> or <link=OrganicPage><color=#0080FFFF>Seeds</color></link>. It has two modes Stabilize and Destabilize.\nStabilize: Increases all genes stability by 50%.\nDestabilize: Decreases all gene stability by 10% other than a chosen gene which will received decreased stability by 50%.\n      ",
            value = "871432335"
        )
    )]
    AppliancePlantGeneticStabilizer = 871432335i32,
    #[strum(serialize = "ItemRoadFlare")]
    #[strum(
        props(
            name = "Road Flare",
            desc = "Designed to burn anywhere in the Solar System, the EZC magnesium fusee supplies its own oxygen to fuel combustion, and dispel the eternal night of space.",
            value = "871811564"
        )
    )]
    ItemRoadFlare = 871811564i32,
    #[strum(serialize = "CartridgeGuide")]
    #[strum(props(name = "Guide", desc = "", value = "872720793"))]
    CartridgeGuide = 872720793i32,
    #[strum(serialize = "StructureLogicSorter")]
    #[strum(
        props(
            name = "Logic Sorter",
            desc = "Contains an Internal Memory which is assessed to check whether something should be sorted. When an item is in the <link=SlotImport><color=orange>Import</color></link> Slot, the stack is checked and if result is true the thing is moved to the <link=SlotExport2><color=orange>Export 2</color></link> slot, otherwise it is moved to the <link=SlotExport><color=orange>Export</color></link> slot. The Mode is used in how the stack is assessed, by default the mode is ALL, so every instruction in the stack would need to return true.",
            value = "873418029"
        )
    )]
    StructureLogicSorter = 873418029i32,
    #[strum(serialize = "StructureLogicRocketDownlink")]
    #[strum(props(name = "Logic Rocket Downlink", desc = "", value = "876108549"))]
    StructureLogicRocketDownlink = 876108549i32,
    #[strum(serialize = "StructureSign1x1")]
    #[strum(props(name = "Sign 1x1", desc = "", value = "879058460"))]
    StructureSign1X1 = 879058460i32,
    #[strum(serialize = "ItemKitLocker")]
    #[strum(props(name = "Kit (Locker)", desc = "", value = "882301399"))]
    ItemKitLocker = 882301399i32,
    #[strum(serialize = "StructureCompositeFloorGratingOpenRotated")]
    #[strum(
        props(
            name = "Composite Floor Grating Open Rotated",
            desc = "",
            value = "882307910"
        )
    )]
    StructureCompositeFloorGratingOpenRotated = 882307910i32,
    #[strum(serialize = "StructureWaterPurifier")]
    #[strum(
        props(
            name = "Water Purifier",
            desc = "Cleans <link=GasPollutedWater><color=#44AD83>Polluted Water</color></link> and outputs <link=GasWater><color=#44AD83>Water</color></link>. The purification process requires <link=ThingItemCharcoal><color=green>Charcoal</color></link> which can be added to the machine via the import bin. The procesing throughput can be improved by increasing the gas pressure of the input pipe relative to the gas pressure of the output pipe.",
            value = "887383294"
        )
    )]
    StructureWaterPurifier = 887383294i32,
    #[strum(serialize = "ItemIgniter")]
    #[strum(
        props(
            name = "Kit (Igniter)",
            desc = "This kit creates an <link=ThingItemIgniter><color=green>Kit (Igniter)</color></link> unit.",
            value = "890106742"
        )
    )]
    ItemIgniter = 890106742i32,
    #[strum(serialize = "ItemFern")]
    #[strum(
        props(
            name = "Fern",
            desc = "There was a time, when Stationeers had to make <link=ThingReagentFenoxitone><color=green>Fenoxitone Powder</color></link> using the <link=ThingApplianceReagentProcessor><color=green>Reagent Processor</color></link>. Recent advances in technology allow you to use equivalent quantities of fern directly in recipes.",
            value = "892110467"
        )
    )]
    ItemFern = 892110467i32,
    #[strum(serialize = "ItemBreadLoaf")]
    #[strum(props(name = "Bread Loaf", desc = "", value = "893514943"))]
    ItemBreadLoaf = 893514943i32,
    #[strum(serialize = "StructureCableJunction5")]
    #[strum(props(name = "Cable (5-Way Junction)", desc = "", value = "894390004"))]
    StructureCableJunction5 = 894390004i32,
    #[strum(serialize = "ItemInsulation")]
    #[strum(
        props(
            name = "Insulation",
            desc = "Mysterious in the extreme, the function of this item is lost to the ages.",
            value = "897176943"
        )
    )]
    ItemInsulation = 897176943i32,
    #[strum(serialize = "StructureWallFlatCornerRound")]
    #[strum(props(name = "Wall (Flat Corner Round)", desc = "", value = "898708250"))]
    StructureWallFlatCornerRound = 898708250i32,
    #[strum(serialize = "ItemHardMiningBackPack")]
    #[strum(props(name = "Hard Mining Backpack", desc = "", value = "900366130"))]
    ItemHardMiningBackPack = 900366130i32,
    #[strum(serialize = "ItemDirtCanister")]
    #[strum(
        props(
            name = "Dirt Canister",
            desc = "A container the will fill with Dirt when using a <link=ThingItemMiningDrill><color=green>Mining Drill</color></link> when placed inside a <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>. You can then use this <link=SlotDirtCanister><color=orange>Dirt Canister</color></link> with the <link=ThingItemTerrainManipulator><color=green>Terrain Manipulator</color></link> to adjust the terrain to suit your needs.",
            value = "902565329"
        )
    )]
    ItemDirtCanister = 902565329i32,
    #[strum(serialize = "StructureSign2x1")]
    #[strum(props(name = "Sign 2x1", desc = "", value = "908320837"))]
    StructureSign2X1 = 908320837i32,
    #[strum(serialize = "CircuitboardAirlockControl")]
    #[strum(
        props(
            name = "Airlock",
            desc = "Rumored to have been first sketched on a <link=Norsec><color=#0080FFFF>Norsec</color></link> toilet wall by a disgruntled engineer, the Exgress airlock control circuit board’s versatility and ease of fabrication has made it the <link=Stationeers><color=#0080FFFF>Stationeers</color></link> control system of choice for <link=ThingStructureAirlock><color=green>Airlock</color></link> cycling protocols. \n\nTo enter setup mode, insert the board into a <link=ThingStructureConsole><color=green>Console</color></link> along with a data disk. In this mode, you can see all data-accessible objects currently connected to the <link=ThingStructureConsole><color=green>Console</color></link>. Doors, lights, gas sensors and slave consoles can be selected (highlighted green), and will be controlled once the data disk is removed.",
            value = "912176135"
        )
    )]
    CircuitboardAirlockControl = 912176135i32,
    #[strum(serialize = "Landingpad_BlankPiece")]
    #[strum(props(name = "Landingpad", desc = "", value = "912453390"))]
    LandingpadBlankPiece = 912453390i32,
    #[strum(serialize = "ItemKitPipeRadiator")]
    #[strum(props(name = "Kit (Pipe Radiator)", desc = "", value = "920411066"))]
    ItemKitPipeRadiator = 920411066i32,
    #[strum(serialize = "StructureLogicMinMax")]
    #[strum(
        props(name = "Logic Min/Max", desc = "0.Greater\n1.Less", value = "929022276")
    )]
    StructureLogicMinMax = 929022276i32,
    #[strum(serialize = "StructureSolarPanel45Reinforced")]
    #[strum(
        props(
            name = "Solar Panel (Heavy Angled)",
            desc = "This solar panel is resistant to storm damage.",
            value = "930865127"
        )
    )]
    StructureSolarPanel45Reinforced = 930865127i32,
    #[strum(serialize = "StructurePoweredVent")]
    #[strum(
        props(
            name = "Powered Vent",
            desc = "Great for moving large quantities of air into a pipe network. Its primary purpose is for the creation of multi-grid airlocks. It can effeciently pull a vacuum on a small to medium sized room.",
            value = "938836756"
        )
    )]
    StructurePoweredVent = 938836756i32,
    #[strum(serialize = "ItemPureIceHydrogen")]
    #[strum(
        props(
            name = "Pure Ice Hydrogen",
            desc = "A frozen chunk of pure <link=GasHydrogen><color=#44AD83>Hydrogen</color></link>",
            value = "944530361"
        )
    )]
    ItemPureIceHydrogen = 944530361i32,
    #[strum(serialize = "StructureHeatExchangeLiquidtoGas")]
    #[strum(
        props(
            name = "Heat Exchanger - Liquid + Gas",
            desc = "The original specs for the N Series Flow-P heat exchanger were rumored to have been scrawled on the back of a burger receipt by a bored <link=Sinotai><color=#0080FFFF>Sinotai</color></link> designer riding up the Brazilian space elevator, but that hasn't stopped it becoming one of the most widely-copied heat exchanger designs in the Solar System.\nThe 'N Flow-P' has four connections, allowing you to pass separate liquid and gas networks into the unit, which then works to equalize temperature across the two separate networks.\nAs the N Flow-P is a passive system, it equalizes pressure across the entire of each individual network, unless connected to devices like a <link=ThingStructureVolumePump><color=green>Volume Pump</color></link> or a <link=ThingStructureBackLiquidPressureRegulator><color=green>Liquid Back Volume Regulator</color></link>.",
            value = "944685608"
        )
    )]
    StructureHeatExchangeLiquidtoGas = 944685608i32,
    #[strum(serialize = "StructureCompositeCladdingAngledCornerInnerLongL")]
    #[strum(
        props(
            name = "Composite Cladding (Angled Corner Inner Long L)",
            desc = "",
            value = "947705066"
        )
    )]
    StructureCompositeCladdingAngledCornerInnerLongL = 947705066i32,
    #[strum(serialize = "StructurePictureFrameThickMountLandscapeLarge")]
    #[strum(
        props(
            name = "Picture Frame Thick Landscape Large",
            desc = "",
            value = "950004659"
        )
    )]
    StructurePictureFrameThickMountLandscapeLarge = 950004659i32,
    #[strum(serialize = "StructureTankSmallAir")]
    #[strum(props(name = "Small Tank (Air)", desc = "", value = "955744474"))]
    StructureTankSmallAir = 955744474i32,
    #[strum(serialize = "StructureHarvie")]
    #[strum(
        props(
            name = "Harvie",
            desc = "Use above a <link=ThingStructureHydroponicsTray><color=green>Hydroponics Tray</color></link> or <link=ThingStructureHydroponicsTrayData><color=green>Hydroponics Device</color></link> to manage the planting and harvest of your crops. It contains a button that will allow you to activate it's modes, or connect it to a logic system to do this for you. The modes indicate current growth status of the plant below. Import is used for planting, and harvested plants are sent to export.",
            value = "958056199"
        )
    )]
    StructureHarvie = 958056199i32,
    #[strum(serialize = "StructureFridgeBig")]
    #[strum(
        props(
            name = "Fridge (Large)",
            desc = "The <link=Xigo><color=#0080FFFF>Xigo</color></link> Koolaid fridge is a self-cooling storage device with 15 slots that preserves food when powered and turned on. While many users have complained about the placement of the power switch, its place in the pantheon of off-world whiteware is unquestioned.\n      \nWith its own permanent internal atmosphere, the Koolaid fridge slows the decay of <link=OrganicPage><color=#0080FFFF>food</color></link> by maintaining an optimal internal temperature. Its power usage varies on the external temperature against which it must balance its internal temperature. As such, it must shed heat to operate, so the Koolaid fridge DOES NOT work in a vacuum.\n      \nAlso, don't leave the door open, as it will equalize with the current world temperature. And maybe start to beep.\n\nFor more information about food preservation, visit the <link=DecayPage><color=#0080FFFF>food decay</color></link> section of the Stationpedia.",
            value = "958476921"
        )
    )]
    StructureFridgeBig = 958476921i32,
    #[strum(serialize = "ItemKitAirlock")]
    #[strum(props(name = "Kit (Airlock)", desc = "", value = "964043875"))]
    ItemKitAirlock = 964043875i32,
    #[strum(serialize = "EntityRoosterBlack")]
    #[strum(
        props(
            name = "Entity Rooster Black",
            desc = "This is a rooster. It is black. There is dignity in this.",
            value = "966959649"
        )
    )]
    EntityRoosterBlack = 966959649i32,
    #[strum(serialize = "ItemKitSorter")]
    #[strum(props(name = "Kit (Sorter)", desc = "", value = "969522478"))]
    ItemKitSorter = 969522478i32,
    #[strum(serialize = "ItemEmergencyCrowbar")]
    #[strum(props(name = "Emergency Crowbar", desc = "", value = "976699731"))]
    ItemEmergencyCrowbar = 976699731i32,
    #[strum(serialize = "Landingpad_DiagonalPiece01")]
    #[strum(
        props(
            name = "Landingpad Diagonal",
            desc = "Extends the size of the landing pad area. A basic trader shuttle requires a 3x3 clear landing area.",
            value = "977899131"
        )
    )]
    LandingpadDiagonalPiece01 = 977899131i32,
    #[strum(serialize = "ReagentColorBlue")]
    #[strum(props(name = "Color Dye (Blue)", desc = "", value = "980054869"))]
    ReagentColorBlue = 980054869i32,
    #[strum(serialize = "StructureCableCorner3")]
    #[strum(
        props(
            name = "Cable (3-Way Corner)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life - so essential, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link> during the 3rd Decannual Stationeer Solar Conference.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "980469101"
        )
    )]
    StructureCableCorner3 = 980469101i32,
    #[strum(serialize = "ItemNVG")]
    #[strum(props(name = "Night Vision Goggles", desc = "", value = "982514123"))]
    ItemNvg = 982514123i32,
    #[strum(serialize = "StructurePlinth")]
    #[strum(props(name = "Plinth", desc = "", value = "989835703"))]
    StructurePlinth = 989835703i32,
    #[strum(serialize = "ItemSprayCanYellow")]
    #[strum(
        props(
            name = "Spray Paint (Yellow)",
            desc = "A caricature of light itself, yellow lacks the self-confidence of red, or the swagger of purple. It's less fun than orange, but less emotionally limp than khaki. It's hard to know when yellow is appropriate, but it persists as a primary color regardless. Suggesting that yellow gonna yellow, no matter what anyone thinks.",
            value = "995468116"
        )
    )]
    ItemSprayCanYellow = 995468116i32,
    #[strum(serialize = "StructureRocketCelestialTracker")]
    #[strum(
        props(
            name = "Rocket Celestial Tracker",
            desc = "The Celestial Tracker can be placed in Rockets and when turned on will provide data that can be used to orientate devices such as the <link=ThingStructureGroundBasedTelescope><color=green>Telescope</color></link>. The Horizontal and Vertical output is localized to the orientation of the tracker. You can calibrate your alignment by comparing the result for the primary body with the output from the <link=ThingStructureDaylightSensor><color=green>Daylight Sensor</color></link>. Full functionality will only be available in orbit, but you can configure using the primary body. For aligning with the telescope, have the face plate facing up and the cables facing in the same direction as for the telescope and the output values will be aligned.",
            value = "997453927"
        )
    )]
    StructureRocketCelestialTracker = 997453927i32,
    #[strum(serialize = "ItemHighVolumeGasCanisterEmpty")]
    #[strum(props(name = "High Volume Gas Canister", desc = "", value = "998653377"))]
    ItemHighVolumeGasCanisterEmpty = 998653377i32,
    #[strum(serialize = "ItemKitLogicTransmitter")]
    #[strum(props(name = "Kit (Logic Transmitter)", desc = "", value = "1005397063"))]
    ItemKitLogicTransmitter = 1005397063i32,
    #[strum(serialize = "StructureIgniter")]
    #[strum(
        props(
            name = "Igniter",
            desc = "It gets the party started. Especially if that party is an explosive gas mixture.",
            value = "1005491513"
        )
    )]
    StructureIgniter = 1005491513i32,
    #[strum(serialize = "SeedBag_Potato")]
    #[strum(
        props(
            name = "Potato Seeds",
            desc = "Grow a <link=ThingItemPotato><color=green>Potato</color></link>.",
            value = "1005571172"
        )
    )]
    SeedBagPotato = 1005571172i32,
    #[strum(serialize = "ItemDataDisk")]
    #[strum(props(name = "Data Disk", desc = "", value = "1005843700"))]
    ItemDataDisk = 1005843700i32,
    #[strum(serialize = "ItemBatteryChargerSmall")]
    #[strum(props(name = "Battery Charger Small", desc = "", value = "1008295833"))]
    ItemBatteryChargerSmall = 1008295833i32,
    #[strum(serialize = "EntityChickenWhite")]
    #[strum(
        props(
            name = "Entity Chicken White",
            desc = "It's a chicken, as white as moondust. It will eat soybeans, corn, and wheat, and lay eggs. Some will be fertilized, producing further chickens. Some will not.",
            value = "1010807532"
        )
    )]
    EntityChickenWhite = 1010807532i32,
    #[strum(serialize = "ItemKitStacker")]
    #[strum(props(name = "Kit (Stacker)", desc = "", value = "1013244511"))]
    ItemKitStacker = 1013244511i32,
    #[strum(serialize = "StructureTankSmall")]
    #[strum(props(name = "Small Tank", desc = "", value = "1013514688"))]
    StructureTankSmall = 1013514688i32,
    #[strum(serialize = "ItemEmptyCan")]
    #[strum(
        props(
            name = "Empty Can",
            desc = "Used for making soups when combined with food in the <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link> or <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link>. Fairly high in nutrition, canned food does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "1013818348"
        )
    )]
    ItemEmptyCan = 1013818348i32,
    #[strum(serialize = "ItemKitTankInsulated")]
    #[strum(props(name = "Kit (Tank Insulated)", desc = "", value = "1021053608"))]
    ItemKitTankInsulated = 1021053608i32,
    #[strum(serialize = "ItemKitChute")]
    #[strum(props(name = "Kit (Basic Chutes)", desc = "", value = "1025254665"))]
    ItemKitChute = 1025254665i32,
    #[strum(serialize = "StructureFuselageTypeA1")]
    #[strum(props(name = "Fuselage (Type A1)", desc = "", value = "1033024712"))]
    StructureFuselageTypeA1 = 1033024712i32,
    #[strum(serialize = "StructureCableAnalysizer")]
    #[strum(props(name = "Cable Analyzer", desc = "", value = "1036015121"))]
    StructureCableAnalysizer = 1036015121i32,
    #[strum(serialize = "StructureCableJunctionH6")]
    #[strum(
        props(name = "Heavy Cable (6-Way Junction)", desc = "", value = "1036780772")
    )]
    StructureCableJunctionH6 = 1036780772i32,
    #[strum(serialize = "ItemGasFilterVolatilesM")]
    #[strum(props(name = "Medium Filter (Volatiles)", desc = "", value = "1037507240"))]
    ItemGasFilterVolatilesM = 1037507240i32,
    #[strum(serialize = "ItemKitPortablesConnector")]
    #[strum(props(name = "Kit (Portables Connector)", desc = "", value = "1041148999"))]
    ItemKitPortablesConnector = 1041148999i32,
    #[strum(serialize = "StructureFloorDrain")]
    #[strum(
        props(
            name = "Passive Liquid Inlet",
            desc = "A passive liquid floor inlet that quickly removes liquids in one direction from the world into the connected pipe network. It will equalise gasses with the world atmosphere also.",
            value = "1048813293"
        )
    )]
    StructureFloorDrain = 1048813293i32,
    #[strum(serialize = "StructureWallGeometryStreight")]
    #[strum(props(name = "Wall (Geometry Straight)", desc = "", value = "1049735537"))]
    StructureWallGeometryStreight = 1049735537i32,
    #[strum(serialize = "StructureTransformerSmallReversed")]
    #[strum(
        props(
            name = "Transformer Reversed (Small)",
            desc = "Transformers control the maximum power that will flow down a cable subnetwork, to prevent overloading <link=ElectronicPage><color=#0080FFFF>electrical</color></link> systems. Output on small transformers can be set from 0 to 5000W.\nNote that transformers operate as data isolators, preventing data flowing into any network beyond it.",
            value = "1054059374"
        )
    )]
    StructureTransformerSmallReversed = 1054059374i32,
    #[strum(serialize = "ItemMiningDrill")]
    #[strum(
        props(
            name = "Mining Drill",
            desc = "The handheld 'Topo' tri-cone rotary mining drill was made for one thing: quick digging. Modeled on a classic <link=Recurso><color=#0080FFFF>Recurso</color></link> zero-g design, it functions equally well in vacuum and atmosphere, with cemented carbide bits to increase resilience and bearing life, and reduce spalling. As Jenk Murtons once said, 'The Topo don't stopo.'",
            value = "1055173191"
        )
    )]
    ItemMiningDrill = 1055173191i32,
    #[strum(serialize = "ItemConstantanIngot")]
    #[strum(props(name = "Ingot (Constantan)", desc = "", value = "1058547521"))]
    ItemConstantanIngot = 1058547521i32,
    #[strum(serialize = "StructureInsulatedPipeCrossJunction6")]
    #[strum(
        props(
            name = "Insulated Pipe (6-Way Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "1061164284"
        )
    )]
    StructureInsulatedPipeCrossJunction6 = 1061164284i32,
    #[strum(serialize = "Landingpad_CenterPiece01")]
    #[strum(
        props(
            name = "Landingpad Center",
            desc = "The target point where the trader shuttle will land. Requires a clear view of the sky.",
            value = "1070143159"
        )
    )]
    LandingpadCenterPiece01 = 1070143159i32,
    #[strum(serialize = "StructureHorizontalAutoMiner")]
    #[strum(
        props(
            name = "OGRE",
            desc = "The <link=Recurso><color=#0080FFFF>Recurso</color></link> OGRE (Orthogonal Ground Rotating Excavator) is a base structure with attached mining vehicle, which will mine a horizontal shaft up to X meters long. When full, the mining vehicle will return to the base to empty itself, before returning to dig. If it encounters empty space, it will also return to base and await instruction. The unit will return if deactivated.\n      \nThe OGRE can be connected to a chute system, and is controllable by a logic network. Note that the OGRE outputs more <link=OrePage><color=#0080FFFF>ore</color></link> than a conventional <link=ThingItemMiningDrill><color=green>Mining Drill</color></link> over the same area, due to more efficient processing.\n\n<size=120%><b>MODES</b></size>\nIdle - 0\nMining - 1\nReturning - 2\nDepostingOre - 3\nFinished - 4\n",
            value = "1070427573"
        )
    )]
    StructureHorizontalAutoMiner = 1070427573i32,
    #[strum(serialize = "ItemDynamicAirCon")]
    #[strum(
        props(name = "Kit (Portable Air Conditioner)", desc = "", value = "1072914031")
    )]
    ItemDynamicAirCon = 1072914031i32,
    #[strum(serialize = "ItemMarineHelmet")]
    #[strum(props(name = "Marine Helmet", desc = "", value = "1073631646"))]
    ItemMarineHelmet = 1073631646i32,
    #[strum(serialize = "StructureDaylightSensor")]
    #[strum(
        props(
            name = "Daylight Sensor",
            desc = "Daylight sensors provide data on whether the current region of your base is in sunlight, and report the exact solar angle. Note that the orientation of the sensor alters the reported solar angle, while <link=LogicPage><color=#0080FFFF>Logic</color></link> systems can be used to offset it.",
            value = "1076425094"
        )
    )]
    StructureDaylightSensor = 1076425094i32,
    #[strum(serialize = "StructureCompositeCladdingCylindricalPanel")]
    #[strum(
        props(
            name = "Composite Cladding (Cylindrical Panel)",
            desc = "",
            value = "1077151132"
        )
    )]
    StructureCompositeCladdingCylindricalPanel = 1077151132i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadMineral")]
    #[strum(
        props(name = "Mining-Drill Head (Mineral)", desc = "", value = "1083675581")
    )]
    ItemRocketMiningDrillHeadMineral = 1083675581i32,
    #[strum(serialize = "ItemKitSuitStorage")]
    #[strum(props(name = "Kit (Suit Storage)", desc = "", value = "1088892825"))]
    ItemKitSuitStorage = 1088892825i32,
    #[strum(serialize = "StructurePictureFrameThinMountPortraitLarge")]
    #[strum(
        props(
            name = "Picture Frame Thin Portrait Large",
            desc = "",
            value = "1094895077"
        )
    )]
    StructurePictureFrameThinMountPortraitLarge = 1094895077i32,
    #[strum(serialize = "StructureLiquidTankBig")]
    #[strum(props(name = "Liquid Tank Big", desc = "", value = "1098900430"))]
    StructureLiquidTankBig = 1098900430i32,
    #[strum(serialize = "Landingpad_CrossPiece")]
    #[strum(
        props(
            name = "Landingpad Cross",
            desc = "Extends the size of the landing pad area. A basic trader shuttle requires a 3x3 clear landing area.",
            value = "1101296153"
        )
    )]
    LandingpadCrossPiece = 1101296153i32,
    #[strum(serialize = "CartridgePlantAnalyser")]
    #[strum(props(name = "Cartridge Plant Analyser", desc = "", value = "1101328282"))]
    CartridgePlantAnalyser = 1101328282i32,
    #[strum(serialize = "ItemSiliconOre")]
    #[strum(
        props(
            name = "Ore (Silicon)",
            desc = "Silicon is a chemical element with the symbol \"Si\" and is one of the most useful elements to <link=Stationeers><color=#0080FFFF>Stationeers</color></link>. Readily available throughout the universe, silicon is used in a range of <link=IngotPage><color=#0080FFFF>alloys</color></link>, glass, plastics and various electronic components a Stationeer may  need to complete their mission.",
            value = "1103972403"
        )
    )]
    ItemSiliconOre = 1103972403i32,
    #[strum(serialize = "ItemWallLight")]
    #[strum(
        props(
            name = "Kit (Lights)",
            desc = "This kit creates any one of ten <link=ThingItemWallLight><color=green>Kit (Lights)</color></link> variants.",
            value = "1108423476"
        )
    )]
    ItemWallLight = 1108423476i32,
    #[strum(serialize = "StructureCableJunction4")]
    #[strum(
        props(
            name = "Cable (4-Way Junction)",
            desc = "Carrying power and data alike, cable coil has come to symbolize the innovation, independence and flexibility of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> life - so much so, the <link=ODA><color=#0080FFFF>ODA</color></link> designated it an official <link=ToolPage><color=#0080FFFF>'tool'</color></link> during the 3rd Decannual Stationeer Solar Conference.\nNormal coil has a maximum wattage of 5kW. For higher-current applications, use <link=ThingItemCableCoilHeavy><color=green>Cable Coil (Heavy)</color></link>.",
            value = "1112047202"
        )
    )]
    StructureCableJunction4 = 1112047202i32,
    #[strum(serialize = "ItemPillHeal")]
    #[strum(
        props(
            name = "Pill (Medical)",
            desc = "Three centuries of pharmaceutical technology compressed into one small, easy to ingest pill: the Heal Pill, aka the Proton Pill, aka Mr Happy contains active enzymes, therapeutic proteins, modified microbial strains, and mammalian cell line analogues in a single-dose boost of high purity, efficacy, and potency that potentiates a swift parasympathetic immune response.",
            value = "1118069417"
        )
    )]
    ItemPillHeal = 1118069417i32,
    #[strum(serialize = "SeedBag_Cocoa")]
    #[strum(props(name = "Cocoa Seeds", desc = "", value = "1139887531"))]
    SeedBagCocoa = 1139887531i32,
    #[strum(serialize = "StructureMediumRocketLiquidFuelTank")]
    #[strum(props(name = "Liquid Capsule Tank Medium", desc = "", value = "1143639539"))]
    StructureMediumRocketLiquidFuelTank = 1143639539i32,
    #[strum(serialize = "StructureCargoStorageMedium")]
    #[strum(props(name = "Cargo Storage (Medium)", desc = "", value = "1151864003"))]
    StructureCargoStorageMedium = 1151864003i32,
    #[strum(serialize = "WeaponRifleEnergy")]
    #[strum(props(name = "Energy Rifle", desc = "0.Stun\n1.Kill", value = "1154745374"))]
    WeaponRifleEnergy = 1154745374i32,
    #[strum(serialize = "StructureSDBSilo")]
    #[strum(
        props(
            name = "SDB Silo",
            desc = "The majestic silo holds large quantities of almost anything. While it is doing that, it cannot be deconstructed. Note also, that any food you put into a silo is likely to <link=DecayPage><color=#0080FFFF>decay</color></link> extremely rapidly. The silo can hold up to 600 stacks.",
            value = "1155865682"
        )
    )]
    StructureSdbSilo = 1155865682i32,
    #[strum(serialize = "Flag_ODA_4m")]
    #[strum(props(name = "Flag (ODA 4m)", desc = "", value = "1159126354"))]
    FlagOda4M = 1159126354i32,
    #[strum(serialize = "ItemCannedPowderedEggs")]
    #[strum(
        props(
            name = "Canned Powdered Eggs",
            desc = "Made in an <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link> or <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link>, using <link=ThingItemCookedPowderedEggs><color=green>Powdered Eggs</color></link>  and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link>, canned powdered eggs are an exciting, dynamic food that's fairly high in nutrition, and does not <link=DecayPage><color=#0080FFFF>decay</color></link>.",
            value = "1161510063"
        )
    )]
    ItemCannedPowderedEggs = 1161510063i32,
    #[strum(serialize = "ItemKitFurniture")]
    #[strum(props(name = "Kit (Furniture)", desc = "", value = "1162905029"))]
    ItemKitFurniture = 1162905029i32,
    #[strum(serialize = "StructureGasGenerator")]
    #[strum(props(name = "Gas Fuel Generator", desc = "", value = "1165997963"))]
    StructureGasGenerator = 1165997963i32,
    #[strum(serialize = "StructureChair")]
    #[strum(
        props(
            name = "Chair",
            desc = "One of the universe's many chairs, optimized for bipeds with somewhere between zero and two upper limbs.",
            value = "1167659360"
        )
    )]
    StructureChair = 1167659360i32,
    #[strum(serialize = "StructureWallPaddedArchLightFittingTop")]
    #[strum(
        props(
            name = "Wall (Padded Arch Light Fitting Top)",
            desc = "",
            value = "1171987947"
        )
    )]
    StructureWallPaddedArchLightFittingTop = 1171987947i32,
    #[strum(serialize = "StructureShelf")]
    #[strum(props(name = "Shelf", desc = "", value = "1172114950"))]
    StructureShelf = 1172114950i32,
    #[strum(serialize = "ApplianceDeskLampRight")]
    #[strum(props(name = "Appliance Desk Lamp Right", desc = "", value = "1174360780"))]
    ApplianceDeskLampRight = 1174360780i32,
    #[strum(serialize = "ItemKitRegulator")]
    #[strum(props(name = "Kit (Pressure Regulator)", desc = "", value = "1181371795"))]
    ItemKitRegulator = 1181371795i32,
    #[strum(serialize = "ItemKitCompositeFloorGrating")]
    #[strum(props(name = "Kit (Floor Grating)", desc = "", value = "1182412869"))]
    ItemKitCompositeFloorGrating = 1182412869i32,
    #[strum(serialize = "StructureWallArchPlating")]
    #[strum(props(name = "Wall (Arch Plating)", desc = "", value = "1182510648"))]
    StructureWallArchPlating = 1182510648i32,
    #[strum(serialize = "StructureWallPaddedCornerThin")]
    #[strum(props(name = "Wall (Padded Corner Thin)", desc = "", value = "1183203913"))]
    StructureWallPaddedCornerThin = 1183203913i32,
    #[strum(serialize = "StructurePowerTransmitterReceiver")]
    #[strum(
        props(
            name = "Microwave Power Receiver",
            desc = "The <link=Norsec><color=#0080FFFF>Norsec</color></link> Wireless Power Transmitter is an uni-directional, A-to-B, far field microwave electrical transmission system.The rotatable base transmitter delivers a narrow, non-lethal microwave beam to a dedicated base receiver.\nThe transmitter must be aligned to the base station in order to transmit any power. The brightness of the transmitter's collimator arc provides an indication of transmission intensity. Note that there is an attrition over longer ranges, so the unit requires more power over greater distances to deliver the same output.Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "1195820278"
        )
    )]
    StructurePowerTransmitterReceiver = 1195820278i32,
    #[strum(serialize = "ItemPipeMeter")]
    #[strum(
        props(
            name = "Kit (Pipe Meter)",
            desc = "This kit creates a <link=ThingStructurePipeMeter><color=green>Pipe Meter</color></link>.",
            value = "1207939683"
        )
    )]
    ItemPipeMeter = 1207939683i32,
    #[strum(serialize = "StructurePictureFrameThinPortraitLarge")]
    #[strum(
        props(
            name = "Picture Frame Thin Portrait Large",
            desc = "",
            value = "1212777087"
        )
    )]
    StructurePictureFrameThinPortraitLarge = 1212777087i32,
    #[strum(serialize = "StructureSleeperLeft")]
    #[strum(
        props(
            name = "Sleeper Left",
            desc = "A horizontal variant of the sleeper. Will keep players hydrated and fed while they are logged out - as long as a breathable atmosphere is provided.",
            value = "1213495833"
        )
    )]
    StructureSleeperLeft = 1213495833i32,
    #[strum(serialize = "ItemIce")]
    #[strum(
        props(
            name = "Ice (Water)",
            desc = "Water ice can be found on most planets in the Solar System, though not all worlds visited by <link=Stationeers><color=#0080FFFF>Stationeers</color></link> possess this resource. Highly sensitive to temperature, ice will begin to melt as soon as it is mined, unless kept in the <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>. When melting, ice produces a mixture of <link=GasSteam><color=#44AD83>Steam</color></link> and <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> gas.",
            value = "1217489948"
        )
    )]
    ItemIce = 1217489948i32,
    #[strum(serialize = "StructureLogicSwitch")]
    #[strum(props(name = "Lever", desc = "", value = "1220484876"))]
    StructureLogicSwitch = 1220484876i32,
    #[strum(serialize = "StructureLiquidUmbilicalFemaleSide")]
    #[strum(
        props(name = "Umbilical Socket Angle (Liquid)", desc = "", value = "1220870319")
    )]
    StructureLiquidUmbilicalFemaleSide = 1220870319i32,
    #[strum(serialize = "ItemKitAtmospherics")]
    #[strum(props(name = "Kit (Atmospherics)", desc = "", value = "1222286371"))]
    ItemKitAtmospherics = 1222286371i32,
    #[strum(serialize = "ItemChemLightYellow")]
    #[strum(
        props(
            name = "Chem Light (Yellow)",
            desc = "Dispel the darkness with this yellow glowstick.",
            value = "1224819963"
        )
    )]
    ItemChemLightYellow = 1224819963i32,
    #[strum(serialize = "ItemIronFrames")]
    #[strum(props(name = "Iron Frames", desc = "", value = "1225836666"))]
    ItemIronFrames = 1225836666i32,
    #[strum(serialize = "CompositeRollCover")]
    #[strum(
        props(
            name = "Composite Roll Cover",
            desc = "0.Operate\n1.Logic",
            value = "1228794916"
        )
    )]
    CompositeRollCover = 1228794916i32,
    #[strum(serialize = "StructureCompositeWall")]
    #[strum(
        props(
            name = "Composite Wall (Type 1)",
            desc = "Air-tight and resistant to extreme temperatures, composite walls favor form over function, coming in a range of slightly different, functionally identical varieties.",
            value = "1237302061"
        )
    )]
    StructureCompositeWall = 1237302061i32,
    #[strum(serialize = "StructureCombustionCentrifuge")]
    #[strum(
        props(
            name = "Combustion Centrifuge",
            desc = "The Combustion Centrifuge is a gas powered version of the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link>. If a <link=ThingStructureRecycler><color=green>Recycler</color></link> or unbalanced <link=ThingStructureFurnace><color=green>Furnace</color></link> outputs <link=ReagentPage><color=#0080FFFF>reagent</color></link> mixture rather than the desired <link=IngotPage><color=#0080FFFF>ingots</color></link>, a centrifuge allows you to reclaim the raw <link=OrePage><color=#0080FFFF>ore</color></link>.\n        It also refines <link=ThingItemDirtyOre><color=green>Dirty Ore</color></link> produced from the <link=ThingStructureDeepMiner><color=green>Deep Miner</color></link> and <link=ThingItemSpaceOre><color=green>Dirty Ore</color></link> produced from the <link=ThingStructureRocketMiner><color=green>Rocket Miner</color></link>. A combustible fuel mix should be supplied to the gas input, and waste gasses should be vented from the output. \n        The machine's RPMs must be controlled via the throttle and combustion limiter levers. If the Combustion Centrifuge gains, or loses, RPMs too fast it will experience stress, and eventually grind to a halt.  Higher RPMs directly result in faster processing speeds. \n        The throttle lever controls the amount of fuel being pulled into the machine, increasing the temperature inside the engine, and leading to an increase in RPM. The limiter lever influences the speed of the combustion, and how much uncombusted gas is in the exhaust. \n        Ejecting ore from the Combustion Centrifuge while it is at high RPMs will result in additional stress build up.  If turned off while not stressed, the machine will automatically start to brake, and reduce RPMs in a controlled manner.\n\t  ",
            value = "1238905683"
        )
    )]
    StructureCombustionCentrifuge = 1238905683i32,
    #[strum(serialize = "ItemVolatiles")]
    #[strum(
        props(
            name = "Ice (Volatiles)",
            desc = "An extremely reactive ice with numerous hydrocarbons trapped inside. For simplicity's sake, these are often displayed as H2 by devices like the <link=ThingCartridgeAtmosAnalyser><color=green>Atmos Analyzer</color></link>.\n      \n<link=GasVolatiles><color=#44AD83>Volatiles</color></link> combust in a 2:1 ratio with <link=GasOxygen><color=#44AD83>Oxygen</color></link>, creating <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> and pollutants. However when catalysed via devices such as the <link=ThingH2Combustor><color=green>H2 Combustor</color></link> in the presence of <link=GasOxygen><color=#44AD83>Oxygen</color></link>, they produce\n        <link=GasSteam><color=#44AD83>Steam</color></link> and heat with a modicum of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> and <link=GasPollutant><color=#44AD83>Pollutant</color></link> due to the autoignition of the volatiles in the chamber. Along with <link=GasOxygen><color=#44AD83>Oxygen</color></link>, volatiles gas is also the major component of fuel for such devices as the <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link>.\n",
            value = "1253102035"
        )
    )]
    ItemVolatiles = 1253102035i32,
    #[strum(serialize = "HandgunMagazine")]
    #[strum(props(name = "Handgun Magazine", desc = "", value = "1254383185"))]
    HandgunMagazine = 1254383185i32,
    #[strum(serialize = "ItemGasFilterVolatilesL")]
    #[strum(props(name = "Heavy Filter (Volatiles)", desc = "", value = "1255156286"))]
    ItemGasFilterVolatilesL = 1255156286i32,
    #[strum(serialize = "ItemMiningDrillPneumatic")]
    #[strum(
        props(
            name = "Pneumatic Mining Drill",
            desc = "0.Default\n1.Flatten",
            value = "1258187304"
        )
    )]
    ItemMiningDrillPneumatic = 1258187304i32,
    #[strum(serialize = "StructureSmallTableDinnerSingle")]
    #[strum(
        props(name = "Small (Table Dinner Single)", desc = "", value = "1260651529")
    )]
    StructureSmallTableDinnerSingle = 1260651529i32,
    #[strum(serialize = "ApplianceReagentProcessor")]
    #[strum(
        props(
            name = "Reagent Processor",
            desc = "Sitting somewhere between a high powered juicer and an alchemist's alembic, the <link=Xigo><color=#0080FFFF>Xigo</color></link> reagent processor turns certain raw materials and food items into cooking and crafting ingredients. Indispensible in any space kitchen, just bolt it to the bench, and you're ready to go.",
            value = "1260918085"
        )
    )]
    ApplianceReagentProcessor = 1260918085i32,
    #[strum(serialize = "StructurePressurePlateMedium")]
    #[strum(props(name = "Trigger Plate (Medium)", desc = "", value = "1269458680"))]
    StructurePressurePlateMedium = 1269458680i32,
    #[strum(serialize = "ItemPumpkin")]
    #[strum(
        props(
            name = "Pumpkin",
            desc = "Pumpkins are a perennial plant, with both a long growth time, and a long time between harvests.  Its low requirement for darkness allows for accelerated growing if provided with extra light.",
            value = "1277828144"
        )
    )]
    ItemPumpkin = 1277828144i32,
    #[strum(serialize = "ItemPumpkinSoup")]
    #[strum(
        props(
            name = "Pumpkin Soup",
            desc = "Made using <link=ThingItemCookedPumpkin><color=green>Cooked Pumpkin</color></link> and an <link=ThingItemEmptyCan><color=green>Empty Can</color></link> in a <link=ThingAppliancePackagingMachine><color=green>Basic Packaging Machine</color></link> or <link=ThingStructureAdvancedPackagingMachine><color=green>Advanced Packaging Machine</color></link>. Fairly high in nutrition, canned food does not <link=DecayPage><color=#0080FFFF>decay</color></link>",
            value = "1277979876"
        )
    )]
    ItemPumpkinSoup = 1277979876i32,
    #[strum(serialize = "StructureTankBigInsulated")]
    #[strum(props(name = "Tank Big (Insulated)", desc = "", value = "1280378227"))]
    StructureTankBigInsulated = 1280378227i32,
    #[strum(serialize = "StructureWallArchCornerTriangle")]
    #[strum(
        props(name = "Wall (Arch Corner Triangle)", desc = "", value = "1281911841")
    )]
    StructureWallArchCornerTriangle = 1281911841i32,
    #[strum(serialize = "StructureTurbineGenerator")]
    #[strum(props(name = "Turbine Generator", desc = "", value = "1282191063"))]
    StructureTurbineGenerator = 1282191063i32,
    #[strum(serialize = "StructurePipeIgniter")]
    #[strum(
        props(
            name = "Pipe Igniter",
            desc = "Ignites the atmosphere inside the attached pipe network.",
            value = "1286441942"
        )
    )]
    StructurePipeIgniter = 1286441942i32,
    #[strum(serialize = "StructureWallIron")]
    #[strum(props(name = "Iron Wall (Type 1)", desc = "", value = "1287324802"))]
    StructureWallIron = 1287324802i32,
    #[strum(serialize = "ItemSprayGun")]
    #[strum(
        props(
            name = "Spray Gun",
            desc = "Use with Spray cans in the <link=SlotSprayCan><color=orange>Spray Can</color></link> to paint structures, cables and pipes. Much more efficient and faster than doing it with individual spray cans.",
            value = "1289723966"
        )
    )]
    ItemSprayGun = 1289723966i32,
    #[strum(serialize = "ItemKitSolidGenerator")]
    #[strum(props(name = "Kit (Solid Generator)", desc = "", value = "1293995736"))]
    ItemKitSolidGenerator = 1293995736i32,
    #[strum(serialize = "StructureAccessBridge")]
    #[strum(
        props(
            name = "Access Bridge",
            desc = "Extendable bridge that spans three grids",
            value = "1298920475"
        )
    )]
    StructureAccessBridge = 1298920475i32,
    #[strum(serialize = "StructurePipeOrgan")]
    #[strum(
        props(
            name = "Pipe Organ",
            desc = "The pipe organ can be attached to one end of a <link=ThingItemPipeValve><color=green>Kit (Pipe Valve)</color></link>. The length of the pipe after the pipe organ changes the pitch of the note it will play when the valve is opened. Use <link=LogicPage><color=#0080FFFF>Logic</color></link> to open and close the valves to create some custom tunes for your base or an audible warning.",
            value = "1305252611"
        )
    )]
    StructurePipeOrgan = 1305252611i32,
    #[strum(serialize = "StructureElectronicsPrinter")]
    #[strum(
        props(
            name = "Electronics Printer",
            desc = "The electronic printer will create any <link=ElectronicPage><color=#0080FFFF>electronic</color></link> part you need. From circuit boards and electronic devices to solar panels. The choice is yours. Upgrade the device using a <link=ThingElectronicPrinterMod><color=green>Electronic Printer Mod</color></link> for additional recipes and faster processing speeds.",
            value = "1307165496"
        )
    )]
    StructureElectronicsPrinter = 1307165496i32,
    #[strum(serialize = "StructureFuselageTypeA4")]
    #[strum(props(name = "Fuselage (Type A4)", desc = "", value = "1308115015"))]
    StructureFuselageTypeA4 = 1308115015i32,
    #[strum(serialize = "StructureSmallDirectHeatExchangeGastoGas")]
    #[strum(
        props(
            name = "Small Direct Heat Exchanger - Gas + Gas",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "1310303582"
        )
    )]
    StructureSmallDirectHeatExchangeGastoGas = 1310303582i32,
    #[strum(serialize = "StructureTurboVolumePump")]
    #[strum(
        props(
            name = "Turbo Volume Pump (Gas)",
            desc = "Shifts 10 times more gas than a basic <link=ThingStructureVolumePump><color=green>Volume Pump</color></link>, with a mode that can be set to flow in either direction.",
            value = "1310794736"
        )
    )]
    StructureTurboVolumePump = 1310794736i32,
    #[strum(serialize = "ItemChemLightWhite")]
    #[strum(
        props(
            name = "Chem Light (White)",
            desc = "Snap the glowstick to activate a pale radiance that keeps the darkness at bay.",
            value = "1312166823"
        )
    )]
    ItemChemLightWhite = 1312166823i32,
    #[strum(serialize = "ItemMilk")]
    #[strum(
        props(
            name = "Milk",
            desc = "Full disclosure, it's not actually 'milk', but an <link=Agrizero><color=#0080FFFF>Agrizero-invented</color></link> synthesis of 5ml <link=ThingItemSoyOil><color=green>Soy Oil</color></link> and 5g <link=ThingItemFern><color=green>Fern</color></link>, delicately blended in the <link=ThingApplianceChemistryStation><color=green>Chemistry Station</color></link>. Surprisingly filling, it can be used as an ingredient to cook other <link=FoodPage><color=#0080FFFF>food</color></link> in the <link=ThingApplianceMicrowave><color=green>Microwave</color></link> or <link=ThingStructureAutomatedOven><color=green>Automated Oven</color></link>. Think, <link=ThingItemMuffin><color=green>Muffin</color></link>.",
            value = "1327248310"
        )
    )]
    ItemMilk = 1327248310i32,
    #[strum(serialize = "StructureInsulatedPipeCrossJunction3")]
    #[strum(
        props(
            name = "Insulated Pipe (3-Way Junction)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "1328210035"
        )
    )]
    StructureInsulatedPipeCrossJunction3 = 1328210035i32,
    #[strum(serialize = "StructureShortCornerLocker")]
    #[strum(props(name = "Short Corner Locker", desc = "", value = "1330754486"))]
    StructureShortCornerLocker = 1330754486i32,
    #[strum(serialize = "StructureTankConnectorLiquid")]
    #[strum(
        props(
            name = "Liquid Tank Connector",
            desc = "These basic mounting devices allow you to attach a <link=ThingDynamicLiquidCanisterEmpty><color=green>Portable Liquid Tank</color></link> to a liquid pipe network.",
            value = "1331802518"
        )
    )]
    StructureTankConnectorLiquid = 1331802518i32,
    #[strum(serialize = "ItemSprayCanPink")]
    #[strum(
        props(
            name = "Spray Paint (Pink)",
            desc = "With the invention of enduring chemical dyes, the 20th century bestowed associations with innocence and tenderness upon this pale tint of red. Yet classically, it was the color of seduction and eroticism. Things change.",
            value = "1344257263"
        )
    )]
    ItemSprayCanPink = 1344257263i32,
    #[strum(serialize = "CircuitboardGraphDisplay")]
    #[strum(props(name = "Graph Display", desc = "", value = "1344368806"))]
    CircuitboardGraphDisplay = 1344368806i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation006")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "1344576960"
        )
    )]
    ItemWreckageStructureWeatherStation006 = 1344576960i32,
    #[strum(serialize = "ItemCookedCorn")]
    #[strum(
        props(
            name = "Cooked Corn",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "1344773148"
        )
    )]
    ItemCookedCorn = 1344773148i32,
    #[strum(serialize = "ItemCookedSoybean")]
    #[strum(
        props(
            name = "Cooked Soybean",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "1353449022"
        )
    )]
    ItemCookedSoybean = 1353449022i32,
    #[strum(serialize = "StructureChuteCorner")]
    #[strum(
        props(
            name = "Chute (Corner)",
            desc = "Chutes act as pipes for items. Use them to connect various <link=ImportExportPage><color=#0080FFFF>import/export</color></link> equipment together such as the <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link> and printers like the <link=ThingStructureAutolathe><color=green>Autolathe</color></link>.\nThe aim for any <link=Stationeers><color=#0080FFFF>Stationeer</color></link> is to make off-world survival less of a struggle for themselves, and those who will follow in their footsteps.\nChute corners are fundamental components of chute networks, which allow the transport of items between machines with <link=ImportExportPage><color=#0080FFFF>import/export</color></link> slots, such as the <link=ThingStructureFurnace><color=green>Furnace</color></link> and other automatable structures.",
            value = "1360330136"
        )
    )]
    StructureChuteCorner = 1360330136i32,
    #[strum(serialize = "DynamicGasCanisterOxygen")]
    #[strum(
        props(
            name = "Portable Gas Tank (Oxygen)",
            desc = "Portable tanks store gas. If you need to refill a tank, bolt it to a <link=ThingItemTankConnector><color=green>Kit (Tank Connector)</color></link> using a <link=ThingItemWrench><color=green>Wrench</color></link>, then connect it to a pipe network. Try to avoid pushing it above 10 MPa, or you'll be picking tank shards out of your face. You can refill a <link=ThingItemGasCanisterOxygen><color=green>Canister (Oxygen)</color></link> by attaching it to the tank's striped section. Or you could vent it into a sealed room to create an atmosphere. Or even paint it pink, call it Steve and fill that sad space in your heart.",
            value = "1360925836"
        )
    )]
    DynamicGasCanisterOxygen = 1360925836i32,
    #[strum(serialize = "StructurePassiveVentInsulated")]
    #[strum(props(name = "Insulated Passive Vent", desc = "", value = "1363077139"))]
    StructurePassiveVentInsulated = 1363077139i32,
    #[strum(serialize = "ApplianceChemistryStation")]
    #[strum(props(name = "Chemistry Station", desc = "", value = "1365789392"))]
    ApplianceChemistryStation = 1365789392i32,
    #[strum(serialize = "ItemPipeIgniter")]
    #[strum(props(name = "Kit (Pipe Igniter)", desc = "", value = "1366030599"))]
    ItemPipeIgniter = 1366030599i32,
    #[strum(serialize = "ItemFries")]
    #[strum(props(name = "French Fries", desc = "", value = "1371786091"))]
    ItemFries = 1371786091i32,
    #[strum(serialize = "StructureSleeperVerticalDroid")]
    #[strum(
        props(
            name = "Droid Sleeper Vertical",
            desc = "The Droid Sleeper will recharge robot batteries and equiped suit batteries if present. This sleeper variant is only safe for robots. Entering as a non robot character will cause you to take damage.",
            value = "1382098999"
        )
    )]
    StructureSleeperVerticalDroid = 1382098999i32,
    #[strum(serialize = "ItemArcWelder")]
    #[strum(props(name = "Arc Welder", desc = "", value = "1385062886"))]
    ItemArcWelder = 1385062886i32,
    #[strum(serialize = "ItemSoyOil")]
    #[strum(props(name = "Soy Oil", desc = "", value = "1387403148"))]
    ItemSoyOil = 1387403148i32,
    #[strum(serialize = "ItemKitRocketAvionics")]
    #[strum(props(name = "Kit (Avionics)", desc = "", value = "1396305045"))]
    ItemKitRocketAvionics = 1396305045i32,
    #[strum(serialize = "ItemMarineBodyArmor")]
    #[strum(props(name = "Marine Armor", desc = "", value = "1399098998"))]
    ItemMarineBodyArmor = 1399098998i32,
    #[strum(serialize = "StructureStairs4x2")]
    #[strum(props(name = "Stairs", desc = "", value = "1405018945"))]
    StructureStairs4X2 = 1405018945i32,
    #[strum(serialize = "ItemKitBattery")]
    #[strum(props(name = "Kit (Battery)", desc = "", value = "1406656973"))]
    ItemKitBattery = 1406656973i32,
    #[strum(serialize = "StructureLargeDirectHeatExchangeGastoLiquid")]
    #[strum(
        props(
            name = "Large Direct Heat Exchanger - Gas + Liquid",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "1412338038"
        )
    )]
    StructureLargeDirectHeatExchangeGastoLiquid = 1412338038i32,
    #[strum(serialize = "AccessCardBrown")]
    #[strum(props(name = "Access Card (Brown)", desc = "", value = "1412428165"))]
    AccessCardBrown = 1412428165i32,
    #[strum(serialize = "StructureCapsuleTankLiquid")]
    #[strum(props(name = "Liquid Capsule Tank Small", desc = "", value = "1415396263"))]
    StructureCapsuleTankLiquid = 1415396263i32,
    #[strum(serialize = "StructureLogicBatchWriter")]
    #[strum(props(name = "Batch Writer", desc = "", value = "1415443359"))]
    StructureLogicBatchWriter = 1415443359i32,
    #[strum(serialize = "StructureCondensationChamber")]
    #[strum(
        props(
            name = "Condensation Chamber",
            desc = "A device for safely condensing gasses into liquids. Liquids and Gasses will both exist safely inside the device. The Chamber will pressurise using its in-built pressure regulator to the target set by the setting wheel.\n        The secondary gas input on the left is a heat-exchanger input and allows for heat exchange between the secondary input pipe and the internal atmosphere of the Condensation Chamber.\n        Paired with <link=ThingStructureEvaporationChamber><color=green>Evaporation Chamber</color></link> Stationeers can exploit the phase change properties of gases to build a DIY air conditioner.",
            value = "1420719315"
        )
    )]
    StructureCondensationChamber = 1420719315i32,
    #[strum(serialize = "SeedBag_Pumpkin")]
    #[strum(
        props(
            name = "Pumpkin Seeds",
            desc = "Grow a <link=ThingItemPumpkin><color=green>Pumpkin</color></link>.",
            value = "1423199840"
        )
    )]
    SeedBagPumpkin = 1423199840i32,
    #[strum(serialize = "ItemPureIceLiquidNitrous")]
    #[strum(
        props(
            name = "Pure Ice Liquid Nitrous",
            desc = "A frozen chunk of pure <link=GasLiquidNitrousOxide><color=#44AD83>Liquid Nitrous Oxide</color></link>",
            value = "1428477399"
        )
    )]
    ItemPureIceLiquidNitrous = 1428477399i32,
    #[strum(serialize = "StructureFrame")]
    #[strum(
        props(
            name = "Steel Frame",
            desc = "More durable than the <link=ThingStructureFrameIron><color=green>Iron Frame</color></link>, steel frames also have several variations for more complex constructions, such as the <link=ThingStructureFrameCorner><color=green>Steel Frame (Corner)</color></link> and <link=ThingStructureFrameCornerCut><color=green>Steel Frame (Corner Cut)</color></link>. Like iron frames, they are placed then completed by welding <link=ThingItemSteelSheets><color=green>Steel Sheets</color></link> to the open framework.",
            value = "1432512808"
        )
    )]
    StructureFrame = 1432512808i32,
    #[strum(serialize = "StructureWaterBottleFillerBottom")]
    #[strum(props(name = "Water Bottle Filler Bottom", desc = "", value = "1433754995"))]
    StructureWaterBottleFillerBottom = 1433754995i32,
    #[strum(serialize = "StructureLightRoundSmall")]
    #[strum(
        props(
            name = "Light Round (Small)",
            desc = "Description coming.",
            value = "1436121888"
        )
    )]
    StructureLightRoundSmall = 1436121888i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadHighSpeedMineral")]
    #[strum(
        props(
            name = "Mining-Drill Head (High Speed Mineral)",
            desc = "",
            value = "1440678625"
        )
    )]
    ItemRocketMiningDrillHeadHighSpeedMineral = 1440678625i32,
    #[strum(serialize = "ItemMKIICrowbar")]
    #[strum(
        props(
            name = "Mk II Crowbar",
            desc = "<link=Recurso><color=#0080FFFF>Recurso's</color></link> entry-level crowbar is useful in a variety of everyday <link=Stationeers><color=#0080FFFF>Stationeer</color></link> settings, from opening <link=ThingStructureAreaPowerControl><color=green>Area Power Control</color></link>s and unpowered <link=ThingStructureAirlock><color=green>Airlock</color></link>s, to splatting pan-dimensional headcrabs, should the need arise. The MK II is more resistant to temperature and pressure.",
            value = "1440775434"
        )
    )]
    ItemMkiiCrowbar = 1440775434i32,
    #[strum(serialize = "StructureHydroponicsStation")]
    #[strum(props(name = "Hydroponics Station", desc = "", value = "1441767298"))]
    StructureHydroponicsStation = 1441767298i32,
    #[strum(serialize = "StructureCryoTubeHorizontal")]
    #[strum(
        props(
            name = "Cryo Tube Horizontal",
            desc = "The horizontal variant of the cryo tube. Will heal players and organs as well as revive dead players when provided with an atmosphere of Nitrogen below -150C.",
            value = "1443059329"
        )
    )]
    StructureCryoTubeHorizontal = 1443059329i32,
    #[strum(serialize = "StructureInsulatedInLineTankLiquid1x2")]
    #[strum(
        props(name = "Insulated In-Line Tank Liquid", desc = "", value = "1452100517")
    )]
    StructureInsulatedInLineTankLiquid1X2 = 1452100517i32,
    #[strum(serialize = "ItemKitPassiveLargeRadiatorLiquid")]
    #[strum(
        props(name = "Kit (Medium Radiator Liquid)", desc = "", value = "1453961898")
    )]
    ItemKitPassiveLargeRadiatorLiquid = 1453961898i32,
    #[strum(serialize = "ItemKitReinforcedWindows")]
    #[strum(props(name = "Kit (Reinforced Windows)", desc = "", value = "1459985302"))]
    ItemKitReinforcedWindows = 1459985302i32,
    #[strum(serialize = "ItemWreckageStructureWeatherStation002")]
    #[strum(
        props(
            name = "Wreckage Structure Weather Station",
            desc = "",
            value = "1464424921"
        )
    )]
    ItemWreckageStructureWeatherStation002 = 1464424921i32,
    #[strum(serialize = "StructureHydroponicsTray")]
    #[strum(
        props(
            name = "Hydroponics Tray",
            desc = "The <link=Agrizero><color=#0080FFFF>Agrizero</color></link> hydroponics tray is the ideal vessel for growing a range of <link=OrganicPage><color=#0080FFFF>plantlife</color></link>. It must be supplied with water using a pipe network, and sufficient light to generate photosynthesis. \nIt can be automated using the <link=ThingStructureHarvie><color=green>Harvie</color></link>.",
            value = "1464854517"
        )
    )]
    StructureHydroponicsTray = 1464854517i32,
    #[strum(serialize = "ItemMkIIToolbelt")]
    #[strum(
        props(
            name = "Tool Belt MK II",
            desc = "A large, ten-slot tool belt with two extra generic slots for carrying whatever takes your fancy.",
            value = "1467558064"
        )
    )]
    ItemMkIiToolbelt = 1467558064i32,
    #[strum(serialize = "StructureOverheadShortLocker")]
    #[strum(props(name = "Overhead Locker", desc = "", value = "1468249454"))]
    StructureOverheadShortLocker = 1468249454i32,
    #[strum(serialize = "ItemMiningBeltMKII")]
    #[strum(
        props(
            name = "Mining Belt MK II",
            desc = "A larger and more capacious mining belt, the Mk II is similar to the <link=ThingItemMiningBelt><color=green>Mining Belt</color></link>, but has 13 slots instead of the basic 8, to increase the length of your mining trips. It also has space for two tools. ",
            value = "1470787934"
        )
    )]
    ItemMiningBeltMkii = 1470787934i32,
    #[strum(serialize = "StructureTorpedoRack")]
    #[strum(props(name = "Torpedo Rack", desc = "", value = "1473807953"))]
    StructureTorpedoRack = 1473807953i32,
    #[strum(serialize = "StructureWallIron02")]
    #[strum(props(name = "Iron Wall (Type 2)", desc = "", value = "1485834215"))]
    StructureWallIron02 = 1485834215i32,
    #[strum(serialize = "StructureWallLargePanel")]
    #[strum(props(name = "Wall (Large Panel)", desc = "", value = "1492930217"))]
    StructureWallLargePanel = 1492930217i32,
    #[strum(serialize = "ItemKitLogicCircuit")]
    #[strum(props(name = "Kit (IC Housing)", desc = "", value = "1512322581"))]
    ItemKitLogicCircuit = 1512322581i32,
    #[strum(serialize = "ItemSprayCanRed")]
    #[strum(
        props(
            name = "Spray Paint (Red)",
            desc = "The king of colors, red is perhaps the defining tone of the universe. Linked to blood, royalty, fire and damnation, it is the chromatic expression of power.",
            value = "1514393921"
        )
    )]
    ItemSprayCanRed = 1514393921i32,
    #[strum(serialize = "StructureLightRound")]
    #[strum(
        props(name = "Light Round", desc = "Description coming.", value = "1514476632")
    )]
    StructureLightRound = 1514476632i32,
    #[strum(serialize = "Fertilizer")]
    #[strum(
        props(
            name = "Fertilizer",
            desc = "Fertilizer alters plant growth processes, and is created by the basic composter and the <link=ThingStructureAdvancedComposter><color=green>Advanced Composter</color></link> using <link=OrganicPage><color=#0080FFFF>organic</color></link> matter.\nFertilizer's affects depend on its ingredients:\n\n- <link=OrganicPage><color=#0080FFFF>Food</color></link> increases PLANT YIELD up to two times\n- <link=ThingDecayedFood><color=green>Decayed Food</color></link> increases plant GROWTH SPEED up to two times\n- <link=ThingItemBiomass><color=green>Biomass</color></link> increases the NUMBER OF GROWTH CYCLES the fertilizer lasts for\n\nThe effect of these ingredients depends on their respective proportions in the composter when processing is activated. ",
            value = "1517856652"
        )
    )]
    Fertilizer = 1517856652i32,
    #[strum(serialize = "StructurePowerUmbilicalMale")]
    #[strum(
        props(
            name = "Umbilical (Power)",
            desc = "0.Left\n1.Center\n2.Right",
            value = "1529453938"
        )
    )]
    StructurePowerUmbilicalMale = 1529453938i32,
    #[strum(serialize = "ItemRocketMiningDrillHeadDurable")]
    #[strum(
        props(name = "Mining-Drill Head (Durable)", desc = "", value = "1530764483")
    )]
    ItemRocketMiningDrillHeadDurable = 1530764483i32,
    #[strum(serialize = "DecayedFood")]
    #[strum(
        props(
            name = "Decayed Food",
            desc = "When your <link=OrganicPage><color=#0080FFFF>food</color></link> decays, it turns into this. <link=ODA><color=#0080FFFF>ODA</color></link> scientists have attempted to determine the exact constituents of this substance, but it remains evasive and mysterious. Suffice to say, eating it is a bad idea. Research has determined, however, that The exact speed of decay varies individually by:\n\n- TEMPERATURE - Refrigeration will slow decay, but many foods will be damaged by exposure to extreme low pressure, as well as extreme heat. The optimum temperature is 0 kelvin (-272 C).\n\n- FOOD TYPE - Each <link=OrganicPage><color=#0080FFFF>food</color></link> type has its own decay properties. <link=ThingItemTomatoSoup><color=green>Tomato Soup</color></link> lasts a lot longer than a <link=ThingItemTomato><color=green>Tomato</color></link>, for instance.\n\n- PRESSURE - Food decays faster when the pressure drops below 1 atmosphere (101kPa). Decay happens exponentially more quickly as the atmosphere approaches a perfect vacuum. There is no effect from higher pressures. \n\n- ATMOSPHERE - Different gases can slow and accelerate the decay process. The process will take account of respective gas ratios in mixed atmospheres in calculating the decay modifier. The following rates apply across all foods:\n\n> <link=GasOxygen><color=#44AD83>Oxygen</color></link> x 1.3\n> <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> x 0.6\n> <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> x 0.8\n> <link=GasVolatiles><color=#44AD83>Volatiles</color></link> x 1\n> <link=GasPollutant><color=#44AD83>Pollutant</color></link> x 3\n> <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> x 1.5\n> <link=GasSteam><color=#44AD83>Steam</color></link> x 2\n> Vacuum (see PRESSURE above)\n\n",
            value = "1531087544"
        )
    )]
    DecayedFood = 1531087544i32,
    #[strum(serialize = "LogicStepSequencer8")]
    #[strum(
        props(
            name = "Logic Step Sequencer",
            desc = "The <link=ODA><color=#0080FFFF>ODA</color></link> does not approve of soundtracks or other distractions.\nAs such, <link=Stationeers><color=#0080FFFF>Stationeers</color></link> have had to create their own musical accompaniment to the demanding labor of building and maintaining off-world infrastructure.\nCentral to this pastime is the step sequencer, which allows Stationeers to sequence short musical patterns or loops. \n\n<size=120%><b>DIY MUSIC - GETTING STARTED</b></size>\n\n1: Connect 8 <link=ThingDeviceStepUnit><color=green>Device Step Unit</color></link>s to your step sequencer via the data port on the left hand side.\n\n2: Label each step unit, then assign step units 1 through 8 on the step sequencer using the screwdriver.\n\n3: Select the output speaker (eg <link=ThingPassiveSpeaker><color=green>Passive Speaker</color></link>) where the sequencer will play the sounds. This needs to be connected to the logic network on the right hand side of the sequencer.\n\n4: Place a <link=ThingStopWatch><color=green>Stop Watch</color></link> and use a <link=ThingStructureLogicReader><color=green>Logic Reader</color></link> and <link=ThingStructureLogicWriter><color=green>Logic Writer</color></link> to write the time to the time variable on the sequencer.\n\n5: Set the BPM on the sequencer using a <link=ThingStructureLogicDial><color=green>Dial</color></link> and a <link=ThingStructureLogicWriter><color=green>Logic Writer</color></link> to write to the sequencer's BPM variable. A higher bpm will play the sequence faster. \n\n6: Insert a sound cartridge of your choosing and select which variant of sound you wish to play by pushing the arrow buttons located above and below the sound cartridge slot.\n\n7: Choose the pitch of the sounds to play by setting the dial on each of your 8 step units to the desired note. With drums, each note is a different drum sounds. You can trial your sounds by pushing the activate button on each step unit (with the sequencer inactive).\n\n8: Get freaky with the <link=ThingDeviceLfoVolume><color=green>Low frequency oscillator</color></link>.\n\n9: Finally, activate the sequencer, Vibeoneer.",
            value = "1531272458"
        )
    )]
    LogicStepSequencer8 = 1531272458i32,
    #[strum(serialize = "ItemKitDynamicGasTankAdvanced")]
    #[strum(
        props(name = "Kit (Portable Gas Tank Mk II)", desc = "", value = "1533501495")
    )]
    ItemKitDynamicGasTankAdvanced = 1533501495i32,
    #[strum(serialize = "ItemWireCutters")]
    #[strum(
        props(
            name = "Wire Cutters",
            desc = "Wirecutters allow you to deconstruct various <link=StructurePage><color=#0080FFFF>structures</color></link>, as well as cross-lay cables when held in your non-active hand, and defuse explosives as needed. Wirecutters are stored in the <link=ThingItemToolBelt><color=green>Tool Belt</color></link>, along with other essential <link=ToolPage><color=#0080FFFF>tools</color></link>.",
            value = "1535854074"
        )
    )]
    ItemWireCutters = 1535854074i32,
    #[strum(serialize = "StructureLadderEnd")]
    #[strum(props(name = "Ladder End", desc = "", value = "1541734993"))]
    StructureLadderEnd = 1541734993i32,
    #[strum(serialize = "ItemGrenade")]
    #[strum(
        props(
            name = "Hand Grenade",
            desc = "Invented by the Romans, who threw Greek Fire at their enemies in ceramic jars, the word 'grenade' is derived from the Old French word for 'pomegranate', as many modern grenades resemble this round, many-seeded fruit. Also like many grenades before it, this one goes boom and breaks stuff.",
            value = "1544275894"
        )
    )]
    ItemGrenade = 1544275894i32,
    #[strum(serialize = "StructureCableJunction5Burnt")]
    #[strum(
        props(name = "Burnt Cable (5-Way Junction)", desc = "", value = "1545286256")
    )]
    StructureCableJunction5Burnt = 1545286256i32,
    #[strum(serialize = "StructurePlatformLadderOpen")]
    #[strum(props(name = "Ladder Platform", desc = "", value = "1559586682"))]
    StructurePlatformLadderOpen = 1559586682i32,
    #[strum(serialize = "StructureTraderWaypoint")]
    #[strum(props(name = "Trader Waypoint", desc = "", value = "1570931620"))]
    StructureTraderWaypoint = 1570931620i32,
    #[strum(serialize = "ItemKitLiquidUmbilical")]
    #[strum(props(name = "Kit (Liquid Umbilical)", desc = "", value = "1571996765"))]
    ItemKitLiquidUmbilical = 1571996765i32,
    #[strum(serialize = "StructureCompositeWall03")]
    #[strum(props(name = "Composite Wall (Type 3)", desc = "", value = "1574321230"))]
    StructureCompositeWall03 = 1574321230i32,
    #[strum(serialize = "ItemKitRespawnPointWallMounted")]
    #[strum(props(name = "Kit (Respawn)", desc = "", value = "1574688481"))]
    ItemKitRespawnPointWallMounted = 1574688481i32,
    #[strum(serialize = "ItemHastelloyIngot")]
    #[strum(props(name = "Ingot (Hastelloy)", desc = "", value = "1579842814"))]
    ItemHastelloyIngot = 1579842814i32,
    #[strum(serialize = "StructurePipeOneWayValve")]
    #[strum(
        props(
            name = "One Way Valve (Gas)",
            desc = "The one way valve moves gas in one direction only: from input side to output side. It only permits flow if the input pressure is higher than output pressure.\n",
            value = "1580412404"
        )
    )]
    StructurePipeOneWayValve = 1580412404i32,
    #[strum(serialize = "StructureStackerReverse")]
    #[strum(
        props(
            name = "Stacker",
            desc = "A stacker is an important part of any automated chute network. The <link=Xigo><color=#0080FFFF>Xigo</color></link> ProKompile can be set manually or via logic, to make sure items passing through the stacker are maximized for your storage needs. The reversed stacker has power and data on the opposite side.\nThe ProKompile can stack a wide variety of things such as <link=IngotPage><color=#0080FFFF>ingots</color></link>, as well as splitting stacks into appropriate sizes as needed.",
            value = "1585641623"
        )
    )]
    StructureStackerReverse = 1585641623i32,
    #[strum(serialize = "ItemKitEvaporationChamber")]
    #[strum(props(name = "Kit (Phase Change Device)", desc = "", value = "1587787610"))]
    ItemKitEvaporationChamber = 1587787610i32,
    #[strum(serialize = "ItemGlassSheets")]
    #[strum(
        props(
            name = "Glass Sheets",
            desc = "A fundamental construction component, glass sheets are created from <link=ReagentSilicon><color=#B566FF>Silicon</color></link>. Fabricated on the <link=ThingStructureAutolathe><color=green>Autolathe</color></link>, they are used to make {THING:StructureSolarPanel;Solar Panels}, <link=ThingStructureAirlock;Airlocks><color=green><N:EN:StructureAirlock;Airlocks></color></link> and many other structures.",
            value = "1588896491"
        )
    )]
    ItemGlassSheets = 1588896491i32,
    #[strum(serialize = "StructureWallPaddedArch")]
    #[strum(props(name = "Wall (Padded Arch)", desc = "", value = "1590330637"))]
    StructureWallPaddedArch = 1590330637i32,
    #[strum(serialize = "StructureLightRoundAngled")]
    #[strum(
        props(
            name = "Light Round (Angled)",
            desc = "Description coming.",
            value = "1592905386"
        )
    )]
    StructureLightRoundAngled = 1592905386i32,
    #[strum(serialize = "StructureWallGeometryT")]
    #[strum(props(name = "Wall (Geometry T)", desc = "", value = "1602758612"))]
    StructureWallGeometryT = 1602758612i32,
    #[strum(serialize = "ItemKitElectricUmbilical")]
    #[strum(props(name = "Kit (Power Umbilical)", desc = "", value = "1603046970"))]
    ItemKitElectricUmbilical = 1603046970i32,
    #[strum(serialize = "Lander")]
    #[strum(props(name = "Lander", desc = "", value = "1605130615"))]
    Lander = 1605130615i32,
    #[strum(serialize = "CartridgeNetworkAnalyser")]
    #[strum(
        props(
            name = "Network Analyzer",
            desc = "A minor masterpiece of micro-electronic engineering, the network analyzer displays the current, voltage and wattage of a cable network, as well as any devices connected to it. Based on a widely-copied <link=Sinotai><color=#0080FFFF>Sinotai</color></link> design, it's used in conjunction with the OreCore <link=ThingItemTablet><color=green>Handheld Tablet</color></link>.",
            value = "1606989119"
        )
    )]
    CartridgeNetworkAnalyser = 1606989119i32,
    #[strum(serialize = "CircuitboardAirControl")]
    #[strum(
        props(
            name = "Air Control",
            desc = "When added to a <link=ThingStructureConsole><color=green>Console</color></link>, air control circuit boards allow you to program an <link=ThingStructureActiveVent><color=green>Active Vent</color></link>. As with small dogs and 83% of people, air control circuits have only three modes: Pressure, Draft and Offline. Pressure mode maintains a 100kPa atmosphere, switching the active vent between inward and outward flow until target pressure is achieved. Draft mode allows you to pair active vents to circulate air. Offline mode deactivates the vent. ",
            value = "1618019559"
        )
    )]
    CircuitboardAirControl = 1618019559i32,
    #[strum(serialize = "StructureUprightWindTurbine")]
    #[strum(
        props(
            name = "Upright Wind Turbine",
            desc = "Norsec's basic wind turbine is an easily fabricated, rapidly deployed design that is strong enough to withstand the worst that environments can throw at it. \nWhile the wind turbine is optimized to produce power even on low atmosphere worlds (up to 200W), it performs best in denser environments. Output varies with wind speed, and during storms, may increase dramatically (up to 800W), so be careful to design your power networks with that in mind.",
            value = "1622183451"
        )
    )]
    StructureUprightWindTurbine = 1622183451i32,
    #[strum(serialize = "StructureFairingTypeA1")]
    #[strum(props(name = "Fairing (Type A1)", desc = "", value = "1622567418"))]
    StructureFairingTypeA1 = 1622567418i32,
    #[strum(serialize = "ItemKitWallArch")]
    #[strum(props(name = "Kit (Arched Wall)", desc = "", value = "1625214531"))]
    ItemKitWallArch = 1625214531i32,
    #[strum(serialize = "StructurePipeLiquidCrossJunction3")]
    #[strum(
        props(
            name = "Liquid Pipe (3-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeLiquidCrossJunction3><color=green><N:EN:StructureInsulatedPipeLiquidCrossJunction3></color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "1628087508"
        )
    )]
    StructurePipeLiquidCrossJunction3 = 1628087508i32,
    #[strum(serialize = "StructureGasTankStorage")]
    #[strum(
        props(
            name = "Gas Tank Storage",
            desc = "When connected to a pipe network, the tank storage unit allows you to refill a <link=ThingItemGasCanisterEmpty><color=green>Canister</color></link>, as well as read various atmospheric data from the <link=SlotGasCanister><color=orange>Gas Canister</color></link>.",
            value = "1632165346"
        )
    )]
    StructureGasTankStorage = 1632165346i32,
    #[strum(serialize = "CircuitboardHashDisplay")]
    #[strum(props(name = "Hash Display", desc = "", value = "1633074601"))]
    CircuitboardHashDisplay = 1633074601i32,
    #[strum(serialize = "CircuitboardAdvAirlockControl")]
    #[strum(props(name = "Advanced Airlock", desc = "", value = "1633663176"))]
    CircuitboardAdvAirlockControl = 1633663176i32,
    #[strum(serialize = "ItemGasFilterCarbonDioxide")]
    #[strum(
        props(
            name = "Filter (Carbon Dioxide)",
            desc = "Given humanity's obsession with exhaling <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link>, all <link=Stationeers><color=#0080FFFF>Stationeers</color></link> are issued two basic <link=Sinotai><color=#0080FFFF>Sinotai</color></link> <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> <link=SlotGasFilter><color=orange>Gas Filter</color></link> as part of their standard deployment kit (SDK). These filters allow passage of <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link> into the suit's waste <link=ThingItemGasCanisterEmpty><color=green>Canister</color></link>, but are also critical components of the <link=ThingDynamicScrubber><color=green>Portable Air Scrubber</color></link> and the <link=ThingStructureFiltration><color=green>Filtration</color></link>. The <link=ThingItemGasFilterCarbonDioxideM><color=green>Medium Filter (Carbon Dioxide)</color></link> and <link=ThingItemGasFilterCarbonDioxideL><color=green>Heavy Filter (Carbon Dioxide)</color></link> are also available.",
            value = "1635000764"
        )
    )]
    ItemGasFilterCarbonDioxide = 1635000764i32,
    #[strum(serialize = "StructureWallFlat")]
    #[strum(props(name = "Wall (Flat)", desc = "", value = "1635864154"))]
    StructureWallFlat = 1635864154i32,
    #[strum(serialize = "StructureChairBoothMiddle")]
    #[strum(props(name = "Chair (Booth Middle)", desc = "", value = "1640720378"))]
    StructureChairBoothMiddle = 1640720378i32,
    #[strum(serialize = "StructureWallArchArrow")]
    #[strum(props(name = "Wall (Arch Arrow)", desc = "", value = "1649708822"))]
    StructureWallArchArrow = 1649708822i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidCrossJunction5")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (5-Way Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "1654694384"
        )
    )]
    StructureInsulatedPipeLiquidCrossJunction5 = 1654694384i32,
    #[strum(serialize = "StructureLogicMath")]
    #[strum(
        props(
            name = "Logic Math",
            desc = "0.Add\n1.Subtract\n2.Multiply\n3.Divide\n4.Mod\n5.Atan2\n6.Pow\n7.Log",
            value = "1657691323"
        )
    )]
    StructureLogicMath = 1657691323i32,
    #[strum(serialize = "ItemKitFridgeSmall")]
    #[strum(props(name = "Kit (Fridge Small)", desc = "", value = "1661226524"))]
    ItemKitFridgeSmall = 1661226524i32,
    #[strum(serialize = "ItemScanner")]
    #[strum(
        props(
            name = "Handheld Scanner",
            desc = "A mysterious piece of technology, rumored to have Zrillian origins.",
            value = "1661270830"
        )
    )]
    ItemScanner = 1661270830i32,
    #[strum(serialize = "ItemEmergencyToolBelt")]
    #[strum(props(name = "Emergency Tool Belt", desc = "", value = "1661941301"))]
    ItemEmergencyToolBelt = 1661941301i32,
    #[strum(serialize = "StructureEmergencyButton")]
    #[strum(
        props(
            name = "Important Button",
            desc = "Description coming.",
            value = "1668452680"
        )
    )]
    StructureEmergencyButton = 1668452680i32,
    #[strum(serialize = "ItemKitAutoMinerSmall")]
    #[strum(props(name = "Kit (Autominer Small)", desc = "", value = "1668815415"))]
    ItemKitAutoMinerSmall = 1668815415i32,
    #[strum(serialize = "StructureChairBacklessSingle")]
    #[strum(props(name = "Chair (Backless Single)", desc = "", value = "1672275150"))]
    StructureChairBacklessSingle = 1672275150i32,
    #[strum(serialize = "ItemPureIceLiquidNitrogen")]
    #[strum(
        props(
            name = "Pure Ice Liquid Nitrogen",
            desc = "A frozen chunk of pure <link=GasLiquidNitrogen><color=#44AD83>Liquid Nitrogen</color></link>",
            value = "1674576569"
        )
    )]
    ItemPureIceLiquidNitrogen = 1674576569i32,
    #[strum(serialize = "ItemEvaSuit")]
    #[strum(
        props(
            name = "Eva Suit",
            desc = "The EVA suit is the basic suit <link=Stationeers><color=#0080FFFF>Stationeers</color></link> need to survive in the inhospitable environment of space. For more information on EVA suits, consult the <link=EVAPage><color=#0080FFFF>EVA suit</color></link> guide.",
            value = "1677018918"
        )
    )]
    ItemEvaSuit = 1677018918i32,
    #[strum(serialize = "StructurePictureFrameThinPortraitSmall")]
    #[strum(
        props(
            name = "Picture Frame Thin Portrait Small",
            desc = "",
            value = "1684488658"
        )
    )]
    StructurePictureFrameThinPortraitSmall = 1684488658i32,
    #[strum(serialize = "StructureLiquidDrain")]
    #[strum(
        props(
            name = "Active Liquid Outlet",
            desc = "When connected to power and activated, it pumps liquid from a liquid network into the world.",
            value = "1687692899"
        )
    )]
    StructureLiquidDrain = 1687692899i32,
    #[strum(serialize = "StructureLiquidTankStorage")]
    #[strum(
        props(
            name = "Liquid Tank Storage",
            desc = "When connected to a liquid pipe network, the tank storage unit allows you to refill a <link=ThingItemLiquidCanisterEmpty><color=green>Liquid Canister</color></link>, as well as read various atmospheric data from the <link=SlotGasCanister><color=orange>Gas Canister</color></link>. It will not accept gas canisters.",
            value = "1691898022"
        )
    )]
    StructureLiquidTankStorage = 1691898022i32,
    #[strum(serialize = "StructurePipeRadiator")]
    #[strum(
        props(
            name = "Pipe Convection Radiator",
            desc = "A simple heat exchanger, pipe radiators can be placed on pipes to shed or gain heat, depending on the temperature of the surrounding atmosphere. If the atmosphere is hotter, heat will be added the gas within the pipe network, and visa versa if colder. In a vacuum, heat will be radiated. \nThe speed of heat gain or loss will depend on the gas in question. Adding multiple radiators will speed up heat transfer.",
            value = "1696603168"
        )
    )]
    StructurePipeRadiator = 1696603168i32,
    #[strum(serialize = "StructureSolarPanelFlatReinforced")]
    #[strum(
        props(
            name = "Solar Panel (Heavy Flat)",
            desc = "This solar panel is resistant to storm damage.",
            value = "1697196770"
        )
    )]
    StructureSolarPanelFlatReinforced = 1697196770i32,
    #[strum(serialize = "ToolPrinterMod")]
    #[strum(
        props(
            name = "Tool Printer Mod",
            desc = "Apply to an <link=ThingStructureToolManufactory><color=green>Tool Manufactory</color></link> with a <link=ThingItemWeldingTorch><color=green>Welding Torch</color></link> or <link=ThingItemArcWelder><color=green>Arc Welder</color></link> to upgrade for increased processing speed and more recipe options.",
            value = "1700018136"
        )
    )]
    ToolPrinterMod = 1700018136i32,
    #[strum(serialize = "StructureCableJunctionH5Burnt")]
    #[strum(
        props(
            name = "Burnt Heavy Cable (5-Way Junction)",
            desc = "",
            value = "1701593300"
        )
    )]
    StructureCableJunctionH5Burnt = 1701593300i32,
    #[strum(serialize = "ItemKitFlagODA")]
    #[strum(props(name = "Kit (ODA Flag)", desc = "", value = "1701764190"))]
    ItemKitFlagOda = 1701764190i32,
    #[strum(serialize = "StructureWallSmallPanelsTwoTone")]
    #[strum(
        props(name = "Wall (Small Panels Two Tone)", desc = "", value = "1709994581")
    )]
    StructureWallSmallPanelsTwoTone = 1709994581i32,
    #[strum(serialize = "ItemFlowerYellow")]
    #[strum(props(name = "Flower (Yellow)", desc = "", value = "1712822019"))]
    ItemFlowerYellow = 1712822019i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidCorner")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (Corner)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "1713710802"
        )
    )]
    StructureInsulatedPipeLiquidCorner = 1713710802i32,
    #[strum(serialize = "ItemCookedCondensedMilk")]
    #[strum(
        props(
            name = "Condensed Milk",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "1715917521"
        )
    )]
    ItemCookedCondensedMilk = 1715917521i32,
    #[strum(serialize = "ItemGasSensor")]
    #[strum(props(name = "Kit (Gas Sensor)", desc = "", value = "1717593480"))]
    ItemGasSensor = 1717593480i32,
    #[strum(serialize = "ItemAdvancedTablet")]
    #[strum(
        props(
            name = "Advanced Tablet",
            desc = "The advanced <link=Xigo><color=#0080FFFF>Xigo</color></link> Padi 2 tablet is an improved version of the basic <link=ThingItemTablet><color=green>Handheld Tablet</color></link>, boasting two <link=CartridgePage><color=#0080FFFF>cartridge</color></link> slots. The Padi 2 accepts <link=ThingCartridgeAtmosAnalyser><color=green>Atmos Analyzer</color></link>, <link=ThingCartridgeTracker><color=green>Tracker</color></link>, <link=ThingCartridgeMedicalAnalyser><color=green>Medical Analyzer</color></link>, <link=ThingCartridgeOreScanner><color=green>Ore Scanner</color></link>, <link=ThingCartridgeElectronicReader><color=green>eReader</color></link>, and various other cartridges.\n\t  \n\t  With a <link=ThingItemIntegratedCircuit10><color=green>Integrated Circuit (IC10)</color></link> in the <link=SlotProgrammableChip><color=orange>Programmable Chip</color></link>, you can access variable slots on the carrying human using the device numbers (d0, d1, etc...), so long as the item can be access via logic, such as the <link=ThingItemHardSuit><color=green>Hardsuit</color></link>.Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>",
            value = "1722785341"
        )
    )]
    ItemAdvancedTablet = 1722785341i32,
    #[strum(serialize = "ItemCoalOre")]
    #[strum(
        props(
            name = "Ore (Coal)",
            desc = "Humanity wouldn't have got to space without humble, combustible coal. Burn it in a <link=ThingSolidFuelGenerator><color=green><N:EN:SolidFuelGenerator></color></link>, smelt it in the <link=ThingStructureFurnace><color=green>Furnace</color></link> to create <link=AlloysPage><color=#0080FFFF>alloys</color></link>, or use it in the <link=ThingApplianceReagentProcessor><color=green>Reagent Processor</color></link> to make  <link=ThingItemSprayCanBlack><color=green>Spray Paint (Black)</color></link>.",
            value = "1724793494"
        )
    )]
    ItemCoalOre = 1724793494i32,
    #[strum(serialize = "EntityChick")]
    #[strum(
        props(
            name = "Entity Chick",
            desc = "Once a chick is hatched, it gets hungry. It will eat soybeans, corn, and wheat, and lay eggs. Some will be fertilized, producing further chickens. Some will not.",
            value = "1730165908"
        )
    )]
    EntityChick = 1730165908i32,
    #[strum(serialize = "StructureLiquidUmbilicalFemale")]
    #[strum(props(name = "Umbilical Socket (Liquid)", desc = "", value = "1734723642"))]
    StructureLiquidUmbilicalFemale = 1734723642i32,
    #[strum(serialize = "StructureAirlockGate")]
    #[strum(
        props(
            name = "Small Hangar Door",
            desc = "1 x 1 modular door piece for building hangar doors.",
            value = "1736080881"
        )
    )]
    StructureAirlockGate = 1736080881i32,
    #[strum(serialize = "CartridgeOreScannerColor")]
    #[strum(
        props(
            name = "Ore Scanner (Color)",
            desc = "When inserted into a <link=ThingItemTablet><color=green>Handheld Tablet</color></link> the scanner will display minerals hidden underground in different colors on the tablet.",
            value = "1738236580"
        )
    )]
    CartridgeOreScannerColor = 1738236580i32,
    #[strum(serialize = "StructureBench4")]
    #[strum(props(name = "Bench (Workbench Style)", desc = "", value = "1750375230"))]
    StructureBench4 = 1750375230i32,
    #[strum(serialize = "StructureCompositeCladdingSphericalCorner")]
    #[strum(
        props(
            name = "Composite Cladding (Spherical Corner)",
            desc = "",
            value = "1751355139"
        )
    )]
    StructureCompositeCladdingSphericalCorner = 1751355139i32,
    #[strum(serialize = "ItemKitRocketScanner")]
    #[strum(props(name = "Kit (Rocket Scanner)", desc = "", value = "1753647154"))]
    ItemKitRocketScanner = 1753647154i32,
    #[strum(serialize = "ItemAreaPowerControl")]
    #[strum(
        props(
            name = "Kit (Power Controller)",
            desc = "This kit places a <link=ThingStructureAreaPowerControl><color=green>Area Power Control</color></link> (APC) on any support structure. The APC kit has two options, selecting which direction you would like the APC power to flow.",
            value = "1757673317"
        )
    )]
    ItemAreaPowerControl = 1757673317i32,
    #[strum(serialize = "ItemIronOre")]
    #[strum(
        props(
            name = "Ore (Iron)",
            desc = "Abundant throughout the Solar System, iron is the <link=OrePage><color=#0080FFFF>ore</color></link> most commonly used by <link=Stationeers><color=#0080FFFF>Stationeers</color></link> constructing offworld bases. It can be smelted into both <link=ThingItemIronIngot><color=green>Ingot (Iron)</color></link>s and <link=ThingItemSteelIngot><color=green>Ingot (Steel)</color></link>s.",
            value = "1758427767"
        )
    )]
    ItemIronOre = 1758427767i32,
    #[strum(serialize = "DeviceStepUnit")]
    #[strum(
        props(
            name = "Device Step Unit",
            desc = "0.C-2\n1.C#-2\n2.D-2\n3.D#-2\n4.E-2\n5.F-2\n6.F#-2\n7.G-2\n8.G#-2\n9.A-2\n10.A#-2\n11.B-2\n12.C-1\n13.C#-1\n14.D-1\n15.D#-1\n16.E-1\n17.F-1\n18.F#-1\n19.G-1\n20.G#-1\n21.A-1\n22.A#-1\n23.B-1\n24.C0\n25.C#0\n26.D0\n27.D#0\n28.E0\n29.F0\n30.F#0\n31.G0\n32.G#0\n33.A0\n34.A#0\n35.B0\n36.C1\n37.C#1\n38.D1\n39.D#1\n40.E1\n41.F1\n42.F#1\n43.G1\n44.G#1\n45.A1\n46.A#1\n47.B1\n48.C2\n49.C#2\n50.D2\n51.D#2\n52.E2\n53.F2\n54.F#2\n55.G2\n56.G#2\n57.A2\n58.A#2\n59.B2\n60.C3\n61.C#3\n62.D3\n63.D#3\n64.E3\n65.F3\n66.F#3\n67.G3\n68.G#3\n69.A3\n70.A#3\n71.B3\n72.C4\n73.C#4\n74.D4\n75.D#4\n76.E4\n77.F4\n78.F#4\n79.G4\n80.G#4\n81.A4\n82.A#4\n83.B4\n84.C5\n85.C#5\n86.D5\n87.D#5\n88.E5\n89.F5\n90.F#5\n91.G5 \n92.G#5\n93.A5\n94.A#5\n95.B5\n96.C6\n97.C#6\n98.D6\n99.D#6\n100.E6\n101.F6\n102.F#6\n103.G6\n104.G#6\n105.A6\n106.A#6\n107.B6\n108.C7\n109.C#7\n110.D7\n111.D#7\n112.E7\n113.F7\n114.F#7\n115.G7\n116.G#7\n117.A7\n118.A#7\n119.B7\n120.C8\n121.C#8\n122.D8\n123.D#8\n124.E8\n125.F8\n126.F#8\n127.G8",
            value = "1762696475"
        )
    )]
    DeviceStepUnit = 1762696475i32,
    #[strum(serialize = "StructureWallPaddedThinNoBorderCorner")]
    #[strum(
        props(
            name = "Wall (Padded Thin No Border Corner)",
            desc = "",
            value = "1769527556"
        )
    )]
    StructureWallPaddedThinNoBorderCorner = 1769527556i32,
    #[strum(serialize = "ItemKitWindowShutter")]
    #[strum(props(name = "Kit (Window Shutter)", desc = "", value = "1779979754"))]
    ItemKitWindowShutter = 1779979754i32,
    #[strum(serialize = "StructureRocketManufactory")]
    #[strum(props(name = "Rocket Manufactory", desc = "", value = "1781051034"))]
    StructureRocketManufactory = 1781051034i32,
    #[strum(serialize = "SeedBag_Soybean")]
    #[strum(
        props(
            name = "Soybean Seeds",
            desc = "Grow some <link=ThingItemSoybean><color=green>Soybean</color></link>.",
            value = "1783004244"
        )
    )]
    SeedBagSoybean = 1783004244i32,
    #[strum(serialize = "ItemEmergencyEvaSuit")]
    #[strum(props(name = "Emergency Eva Suit", desc = "", value = "1791306431"))]
    ItemEmergencyEvaSuit = 1791306431i32,
    #[strum(serialize = "StructureWallArchCornerRound")]
    #[strum(props(name = "Wall (Arch Corner Round)", desc = "", value = "1794588890"))]
    StructureWallArchCornerRound = 1794588890i32,
    #[strum(serialize = "ItemCoffeeMug")]
    #[strum(props(name = "Coffee Mug", desc = "", value = "1800622698"))]
    ItemCoffeeMug = 1800622698i32,
    #[strum(serialize = "StructureAngledBench")]
    #[strum(props(name = "Bench (Angled)", desc = "", value = "1811979158"))]
    StructureAngledBench = 1811979158i32,
    #[strum(serialize = "StructurePassiveLiquidDrain")]
    #[strum(
        props(
            name = "Passive Liquid Drain",
            desc = "Moves liquids from a pipe network to the world atmosphere.",
            value = "1812364811"
        )
    )]
    StructurePassiveLiquidDrain = 1812364811i32,
    #[strum(serialize = "ItemKitLandingPadAtmos")]
    #[strum(
        props(name = "Kit (Landing Pad Atmospherics)", desc = "", value = "1817007843")
    )]
    ItemKitLandingPadAtmos = 1817007843i32,
    #[strum(serialize = "ItemRTGSurvival")]
    #[strum(
        props(
            name = "Kit (RTG)",
            desc = "This kit creates a <link=ThingItemRTGSurvival><color=green>Kit (RTG)</color></link>.",
            value = "1817645803"
        )
    )]
    ItemRtgSurvival = 1817645803i32,
    #[strum(serialize = "StructureInsulatedInLineTankGas1x1")]
    #[strum(
        props(name = "Insulated In-Line Tank Small Gas", desc = "", value = "1818267386")
    )]
    StructureInsulatedInLineTankGas1X1 = 1818267386i32,
    #[strum(serialize = "ItemPlantThermogenic_Genepool2")]
    #[strum(
        props(
            name = "Hades Flower (Beta strain)",
            desc = "The <link=Agrizero><color=#0080FFFF>Agrizero's</color></link>-created Hades Flower is the result of as dubious experiment to combine the allure of tropical plants with the comfort and homeliness of a heat pump. The plant breathes a 1:3 mix of <link=GasVolatiles><color=#44AD83>Volatiles</color></link> and <link=GasOxygen><color=#44AD83>Oxygen</color></link>, and exhales heated <link=GasPollutant><color=#44AD83>Pollutant</color></link>. The beta strain is notably more efficient than the earlier, more experimental alpha variant.",
            value = "1819167057"
        )
    )]
    ItemPlantThermogenicGenepool2 = 1819167057i32,
    #[strum(serialize = "StructureLogicSelect")]
    #[strum(
        props(
            name = "Logic Select",
            desc = "0.Equals\n1.Greater\n2.Less\n3.NotEquals",
            value = "1822736084"
        )
    )]
    StructureLogicSelect = 1822736084i32,
    #[strum(serialize = "ItemGasFilterNitrousOxideM")]
    #[strum(
        props(name = "Medium Filter (Nitrous Oxide)", desc = "", value = "1824284061")
    )]
    ItemGasFilterNitrousOxideM = 1824284061i32,
    #[strum(serialize = "StructureSmallDirectHeatExchangeLiquidtoGas")]
    #[strum(
        props(
            name = "Small Direct Heat Exchanger - Liquid + Gas ",
            desc = "Direct Heat Exchangers equalize the temperature of the two input networks.",
            value = "1825212016"
        )
    )]
    StructureSmallDirectHeatExchangeLiquidtoGas = 1825212016i32,
    #[strum(serialize = "ItemKitRoverFrame")]
    #[strum(props(name = "Kit (Rover Frame)", desc = "", value = "1827215803"))]
    ItemKitRoverFrame = 1827215803i32,
    #[strum(serialize = "ItemNickelOre")]
    #[strum(
        props(
            name = "Ore (Nickel)",
            desc = "Nickel is a chemical element with the symbol \"Ni\" and is a rare metal commonly used as a plating to prevent corrosion. Sought after by many <link=Stationeers><color=#0080FFFF>Stationeers</color></link>, Nickel is also commonly used to create several <link=IngotPage><color=#0080FFFF>alloys</color></link>.",
            value = "1830218956"
        )
    )]
    ItemNickelOre = 1830218956i32,
    #[strum(serialize = "StructurePictureFrameThinMountPortraitSmall")]
    #[strum(
        props(
            name = "Picture Frame Thin Portrait Small",
            desc = "",
            value = "1835796040"
        )
    )]
    StructurePictureFrameThinMountPortraitSmall = 1835796040i32,
    #[strum(serialize = "H2Combustor")]
    #[strum(
        props(
            name = "H2 Combustor",
            desc = "Adapted slightly from its original <link=Recurso><color=#0080FFFF>Recurso</color></link> design, the <link=GasVolatiles><color=#44AD83>Volatiles</color></link> Combustor does exactly what its name suggests - it burns a mixture of volatiles and <link=GasOxygen><color=#44AD83>Oxygen</color></link> to create water. Extremely useful in hot or arid environments, users need to be aware that the combustor outputs considerable waste heat. The device is also less than perfectly efficient, resulting in the autoignition of volatiles in the chamber, and the production of waste gases which must be dealt with.",
            value = "1840108251"
        )
    )]
    H2Combustor = 1840108251i32,
    #[strum(serialize = "Flag_ODA_10m")]
    #[strum(props(name = "Flag (ODA 10m)", desc = "", value = "1845441951"))]
    FlagOda10M = 1845441951i32,
    #[strum(serialize = "StructureLightLongAngled")]
    #[strum(props(name = "Wall Light (Long Angled)", desc = "", value = "1847265835"))]
    StructureLightLongAngled = 1847265835i32,
    #[strum(serialize = "StructurePipeLiquidCrossJunction")]
    #[strum(
        props(
            name = "Liquid Pipe (Cross Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructurePipeInsulatedLiquidCrossJunction><color=green>Insulated Liquid Pipe (Cross Junction)</color></link> using an <link=ThingItemKitInsulatedLiquidPipe><color=green>Kit (Insulated Liquid Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "1848735691"
        )
    )]
    StructurePipeLiquidCrossJunction = 1848735691i32,
    #[strum(serialize = "ItemCookedPumpkin")]
    #[strum(
        props(
            name = "Cooked Pumpkin",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "1849281546"
        )
    )]
    ItemCookedPumpkin = 1849281546i32,
    #[strum(serialize = "StructureLiquidValve")]
    #[strum(props(name = "Liquid Valve", desc = "", value = "1849974453"))]
    StructureLiquidValve = 1849974453i32,
    #[strum(serialize = "ApplianceTabletDock")]
    #[strum(props(name = "Tablet Dock", desc = "", value = "1853941363"))]
    ApplianceTabletDock = 1853941363i32,
    #[strum(serialize = "StructureCableJunction6HBurnt")]
    #[strum(
        props(name = "Burnt Cable (6-Way Junction)", desc = "", value = "1854404029")
    )]
    StructureCableJunction6HBurnt = 1854404029i32,
    #[strum(serialize = "ItemMKIIWrench")]
    #[strum(
        props(
            name = "Mk II Wrench",
            desc = "One of humanity's enduring contributions to the cosmos, the wrench represents the essence of our species. A simple, effective and spiritually barren tool, use it to build and deconstruct a variety of <link=StructurePage><color=#0080FFFF>structures</color></link> The MK II is more resistant to temperature and pressure.",
            value = "1862001680"
        )
    )]
    ItemMkiiWrench = 1862001680i32,
    #[strum(serialize = "ItemAdhesiveInsulation")]
    #[strum(props(name = "Adhesive Insulation", desc = "", value = "1871048978"))]
    ItemAdhesiveInsulation = 1871048978i32,
    #[strum(serialize = "ItemGasFilterCarbonDioxideL")]
    #[strum(
        props(name = "Heavy Filter (Carbon Dioxide)", desc = "", value = "1876847024")
    )]
    ItemGasFilterCarbonDioxideL = 1876847024i32,
    #[strum(serialize = "ItemWallHeater")]
    #[strum(
        props(
            name = "Kit (Wall Heater)",
            desc = "This kit creates a <link=ThingItemWallHeater><color=green>Kit (Wall Heater)</color></link>.",
            value = "1880134612"
        )
    )]
    ItemWallHeater = 1880134612i32,
    #[strum(serialize = "StructureNitrolyzer")]
    #[strum(
        props(
            name = "Nitrolyzer",
            desc = "This device is used to create <link=GasNitrousOxide><color=#44AD83>Nitrous Oxide</color></link> from <link=GasOxygen><color=#44AD83>Oxygen</color></link>, <link=GasNitrogen><color=#44AD83>Nitrogen</color></link>, and a large amount of energy. The process does not completely transform all the available gas at once, so the output is a mix of all three gasses, which may need further processing.  More NOS will be created, if the gas inside the machine is close to a 1/1 ratio of Oxygen to Nitrogen.  The second gas input line in optional, and not required if the gas is pre mixed.",
            value = "1898243702"
        )
    )]
    StructureNitrolyzer = 1898243702i32,
    #[strum(serialize = "StructureLargeSatelliteDish")]
    #[strum(
        props(
            name = "Large Satellite Dish",
            desc = "This large communications unit can be used to communicate with nearby trade vessels.\n\n        When connected to a <link=ThingStructureComputer><color=green>Computer</color></link> containing a <link=ThingMotherboardComms><color=green>Communications Motherboard</color></link> motherboard, a <link=ThingLandingpad_CenterPiece01><color=green>Landingpad Center</color></link>, and a <link=ThingStructureVendingMachine><color=green>Vending Machine</color></link>, this allows Stationeers to contact traders. Adjust its horizontal and vertical attributes either directly or through logic.",
            value = "1913391845"
        )
    )]
    StructureLargeSatelliteDish = 1913391845i32,
    #[strum(serialize = "ItemGasFilterPollutants")]
    #[strum(
        props(
            name = "Filter (Pollutant)",
            desc = "Filters are used to capture various gases, such as waste emissions from a <link=ThingStructureFurnace><color=green>Furnace</color></link> or <link=ThingStructureArcFurnace><color=green>Arc Furnace</color></link>. Adding <link=Sinotai><color=#0080FFFF>Sinotai</color></link>-designed <link=GasPollutant><color=#44AD83>Pollutant</color></link> filters to a <link=ThingItemDynamicScrubber><color=green>Kit (Portable Scrubber)</color></link> allows you to isolate this gas, then add it to a pipe network and employ its excellent coolant properties in a <link=ThingStructureWallCooler><color=green>Wall Cooler</color></link>. Try not to inhale.",
            value = "1915566057"
        )
    )]
    ItemGasFilterPollutants = 1915566057i32,
    #[strum(serialize = "ItemSprayCanKhaki")]
    #[strum(
        props(
            name = "Spray Paint (Khaki)",
            desc = "Not so much a single color, as a category of boredom, khaki is the pigmentation equivalent of a mild depressive episode.",
            value = "1918456047"
        )
    )]
    ItemSprayCanKhaki = 1918456047i32,
    #[strum(serialize = "ItemKitPumpedLiquidEngine")]
    #[strum(props(name = "Kit (Pumped Liquid Engine)", desc = "", value = "1921918951"))]
    ItemKitPumpedLiquidEngine = 1921918951i32,
    #[strum(serialize = "StructurePowerUmbilicalFemaleSide")]
    #[strum(
        props(name = "Umbilical Socket Angle (Power)", desc = "", value = "1922506192")
    )]
    StructurePowerUmbilicalFemaleSide = 1922506192i32,
    #[strum(serialize = "ItemSoybean")]
    #[strum(
        props(
            name = "Soybean",
            desc = " Soybeans grow at a moderate rate, but require atmospheric <link=GasNitrogen><color=#44AD83>Nitrogen</color></link> to grow.  Its main use is to create <link=ThingItemSoyOil><color=green>Soy Oil</color></link>",
            value = "1924673028"
        )
    )]
    ItemSoybean = 1924673028i32,
    #[strum(serialize = "StructureInsulatedPipeLiquidCrossJunction")]
    #[strum(
        props(
            name = "Insulated Liquid Pipe (3-Way Junction)",
            desc = "Liquid piping with very low temperature loss or gain.",
            value = "1926651727"
        )
    )]
    StructureInsulatedPipeLiquidCrossJunction = 1926651727i32,
    #[strum(serialize = "ItemWreckageTurbineGenerator3")]
    #[strum(props(name = "Wreckage Turbine Generator", desc = "", value = "1927790321"))]
    ItemWreckageTurbineGenerator3 = 1927790321i32,
    #[strum(serialize = "StructurePassthroughHeatExchangerGasToLiquid")]
    #[strum(
        props(
            name = "CounterFlow Heat Exchanger - Gas + Liquid",
            desc = "Exchange heat from one pipe network to another. By drawing down the pressure of the outputs with a pump or regulator and regulating input pressures, the temperatures of two counterflowing networks can be effectively exchanged.\n        Balancing the throughput of both inputs is key to creating a good exhange of temperatures.",
            value = "1928991265"
        )
    )]
    StructurePassthroughHeatExchangerGasToLiquid = 1928991265i32,
    #[strum(serialize = "ItemPotato")]
    #[strum(
        props(
            name = "Potato",
            desc = " Potatoes are a simple, fast growing crop that can keep Stationeers alive in emergencies.",
            value = "1929046963"
        )
    )]
    ItemPotato = 1929046963i32,
    #[strum(serialize = "StructureCableCornerHBurnt")]
    #[strum(props(name = "Burnt Cable (Corner)", desc = "", value = "1931412811"))]
    StructureCableCornerHBurnt = 1931412811i32,
    #[strum(serialize = "KitSDBSilo")]
    #[strum(
        props(
            name = "Kit (SDB Silo)",
            desc = "This kit creates a <link=ThingStructureSDBSilo><color=green>SDB Silo</color></link>.",
            value = "1932952652"
        )
    )]
    KitSdbSilo = 1932952652i32,
    #[strum(serialize = "ItemKitPipeUtility")]
    #[strum(props(name = "Kit (Pipe Utility Gas)", desc = "", value = "1934508338"))]
    ItemKitPipeUtility = 1934508338i32,
    #[strum(serialize = "ItemKitInteriorDoors")]
    #[strum(props(name = "Kit (Interior Doors)", desc = "", value = "1935945891"))]
    ItemKitInteriorDoors = 1935945891i32,
    #[strum(serialize = "StructureCryoTube")]
    #[strum(
        props(
            name = "CryoTube",
            desc = "The exact operation of the Longsleep cryotube remains a commercial secret, with <link=Norsec><color=#0080FFFF>Norsec</color></link> merely licensing the design. Able to regenerate organ damage when supplied with power and an atmosphere, the Longsleep is a minor miracle of modern medical technology.",
            value = "1938254586"
        )
    )]
    StructureCryoTube = 1938254586i32,
    #[strum(serialize = "StructureReinforcedWallPaddedWindow")]
    #[strum(
        props(
            name = "Reinforced Window (Padded)",
            desc = "Enjoy vistas of even the most savage, alien landscapes with these heavy duty window frames, which are resistant to pressure differentials up to 1MPa.",
            value = "1939061729"
        )
    )]
    StructureReinforcedWallPaddedWindow = 1939061729i32,
    #[strum(serialize = "DynamicCrate")]
    #[strum(
        props(
            name = "Dynamic Crate",
            desc = "The humble dynamic crate has become a symbol of <link=Stationeers><color=#0080FFFF>Stationeer</color></link> invention and independence. With twelve slots and handles at either end for ease of carriage, it's both standard issue and critical kit for cadets and Commanders alike.",
            value = "1941079206"
        )
    )]
    DynamicCrate = 1941079206i32,
    #[strum(serialize = "StructureLogicGate")]
    #[strum(
        props(
            name = "Logic Gate",
            desc = "A <link=LogicPage><color=#0080FFFF>logic</color></link> device that performs a logical operation on one or more binary inputs that produces a single binary output. An input greater than zero is considered true for operations.",
            value = "1942143074"
        )
    )]
    StructureLogicGate = 1942143074i32,
    #[strum(serialize = "StructureDiode")]
    #[strum(props(name = "LED", desc = "", value = "1944485013"))]
    StructureDiode = 1944485013i32,
    #[strum(serialize = "StructureChairBacklessDouble")]
    #[strum(props(name = "Chair (Backless Double)", desc = "", value = "1944858936"))]
    StructureChairBacklessDouble = 1944858936i32,
    #[strum(serialize = "StructureBatteryCharger")]
    #[strum(
        props(
            name = "Battery Cell Charger",
            desc = "The 5-slot <link=Xigo><color=#0080FFFF>Xigo</color></link> battery charger fits the <link=ThingItemBatteryCell><color=green>Battery Cell (Small)</color></link>, <link=ThingItemBatteryCellLarge><color=green>Battery Cell (Large)</color></link> and <link=ThingItemBatteryCellNuclear><color=green>Battery Cell (Nuclear)</color></link>, providing up to 500W to any connected cell. Note: the older design means this device has minor power draw (10W) even when not charging.",
            value = "1945930022"
        )
    )]
    StructureBatteryCharger = 1945930022i32,
    #[strum(serialize = "StructureFurnace")]
    #[strum(
        props(
            name = "Furnace",
            desc = "The Zhurong furnace employs a high-temperature gas mixture of <link=GasOxygen><color=#44AD83>Oxygen</color></link> and <link=GasVolatiles><color=#44AD83>Volatiles</color></link> to smelt <link=IngotPage><color=#0080FFFF>ingots</color></link> and a range of <link=IngotPage><color=#0080FFFF>alloys</color></link> as raw materials for <link=FabricatorPage><color=#0080FFFF>fabricators</color></link>.\nA basic gas mixture can be achieved by adding <link=ThingItemOxite><color=green>Ice (Oxite)</color></link> and <link=ThingItemVolatiles><color=green>Ice (Volatiles)</color></link> in a 1:2 ratio directly to the furnace, but more complex alloys will require careful management of a dedicated gas mixing network. Exact ingredient ratios must be observed. Likewise, smelting ores at insufficient temperatures will produce <link=ReagentPage><color=#0080FFFF>reagents</color></link>, which must be recycled.\nIf liquids are present in the furnace, they will gather there until the furnace is connected to a liquid pipe network.",
            value = "1947944864"
        )
    )]
    StructureFurnace = 1947944864i32,
    #[strum(serialize = "ItemLightSword")]
    #[strum(
        props(
            name = "Light Sword",
            desc = "A charming, if useless, pseudo-weapon. (Creative only.)",
            value = "1949076595"
        )
    )]
    ItemLightSword = 1949076595i32,
    #[strum(serialize = "ItemKitLiquidRegulator")]
    #[strum(props(name = "Kit (Liquid Regulator)", desc = "", value = "1951126161"))]
    ItemKitLiquidRegulator = 1951126161i32,
    #[strum(serialize = "StructureCompositeCladdingRoundedCorner")]
    #[strum(
        props(
            name = "Composite Cladding (Rounded Corner)",
            desc = "",
            value = "1951525046"
        )
    )]
    StructureCompositeCladdingRoundedCorner = 1951525046i32,
    #[strum(serialize = "ItemGasFilterPollutantsL")]
    #[strum(props(name = "Heavy Filter (Pollutants)", desc = "", value = "1959564765"))]
    ItemGasFilterPollutantsL = 1959564765i32,
    #[strum(serialize = "ItemKitSmallSatelliteDish")]
    #[strum(props(name = "Kit (Small Satellite Dish)", desc = "", value = "1960952220"))]
    ItemKitSmallSatelliteDish = 1960952220i32,
    #[strum(serialize = "StructureSolarPanelFlat")]
    #[strum(
        props(
            name = "Solar Panel (Flat)",
            desc = "<link=Sinotai><color=#0080FFFF>Sinotai</color></link> basic solar panels generate power from sunlight. They lie flat to the ground, and their efficiency is reduced during storms and when damaged. You can repair these using some trusty <link=ThingItemDuctTape><color=green>Duct Tape</color></link>.",
            value = "1968102968"
        )
    )]
    StructureSolarPanelFlat = 1968102968i32,
    #[strum(serialize = "StructureDrinkingFountain")]
    #[strum(
        props(
            name = "<N:EN:StructureDrinkingFountain>",
            desc = "<N:EN:StructureDrinkingFountain>",
            value = "1968371847"
        )
    )]
    StructureDrinkingFountain = 1968371847i32,
    #[strum(serialize = "ItemJetpackBasic")]
    #[strum(
        props(
            name = "Jetpack Basic",
            desc = "The basic <link=CHAC><color=#0080FFFF>CHAC</color></link> jetpack isn't 'technically' a jetpack, it's a gas thruster. It can be powered by any gas, so long as the internal pressure of the <link=AtmosphericsPage><color=#0080FFFF>canister</color></link> is higher than the ambient external pressure. If the external pressure is greater, the spacepack will not function.\nIndispensable for building, mining and general movement, it has ten storage slots and lets <link=Stationeers><color=#0080FFFF>Stationeers</color></link> fly at 3m/s, compared to the more powerful <link=ThingItemHardJetpack><color=green>Hardsuit Jetpack</color></link>. Adjusting the thrust value alters your rate of acceleration, while activating the stabilizer causes the spacepack to hover when a given height is reached.\nUSE: 'J' to activate; 'space' to fly up; 'left ctrl' to descend; and 'WASD' to move.",
            value = "1969189000"
        )
    )]
    ItemJetpackBasic = 1969189000i32,
    #[strum(serialize = "ItemKitEngineMedium")]
    #[strum(props(name = "Kit (Engine Medium)", desc = "", value = "1969312177"))]
    ItemKitEngineMedium = 1969312177i32,
    #[strum(serialize = "StructureWallGeometryCorner")]
    #[strum(props(name = "Wall (Geometry Corner)", desc = "", value = "1979212240"))]
    StructureWallGeometryCorner = 1979212240i32,
    #[strum(serialize = "StructureInteriorDoorPaddedThin")]
    #[strum(
        props(
            name = "Interior Door Padded Thin",
            desc = "0.Operate\n1.Logic",
            value = "1981698201"
        )
    )]
    StructureInteriorDoorPaddedThin = 1981698201i32,
    #[strum(serialize = "StructureWaterBottleFillerPoweredBottom")]
    #[strum(props(name = "Waterbottle Filler", desc = "", value = "1986658780"))]
    StructureWaterBottleFillerPoweredBottom = 1986658780i32,
    #[strum(serialize = "StructureLiquidTankSmall")]
    #[strum(props(name = "Liquid Tank Small", desc = "", value = "1988118157"))]
    StructureLiquidTankSmall = 1988118157i32,
    #[strum(serialize = "ItemKitComputer")]
    #[strum(props(name = "Kit (Computer)", desc = "", value = "1990225489"))]
    ItemKitComputer = 1990225489i32,
    #[strum(serialize = "StructureWeatherStation")]
    #[strum(
        props(
            name = "Weather Station",
            desc = "0.NoStorm\n1.StormIncoming\n2.InStorm",
            value = "1997212478"
        )
    )]
    StructureWeatherStation = 1997212478i32,
    #[strum(serialize = "ItemKitLogicInputOutput")]
    #[strum(props(name = "Kit (Logic I/O)", desc = "", value = "1997293610"))]
    ItemKitLogicInputOutput = 1997293610i32,
    #[strum(serialize = "StructureCompositeCladdingPanel")]
    #[strum(props(name = "Composite Cladding (Panel)", desc = "", value = "1997436771"))]
    StructureCompositeCladdingPanel = 1997436771i32,
    #[strum(serialize = "StructureElevatorShaftIndustrial")]
    #[strum(props(name = "Elevator Shaft", desc = "", value = "1998354978"))]
    StructureElevatorShaftIndustrial = 1998354978i32,
    #[strum(serialize = "ReagentColorRed")]
    #[strum(props(name = "Color Dye (Red)", desc = "", value = "1998377961"))]
    ReagentColorRed = 1998377961i32,
    #[strum(serialize = "Flag_ODA_6m")]
    #[strum(props(name = "Flag (ODA 6m)", desc = "", value = "1998634960"))]
    FlagOda6M = 1998634960i32,
    #[strum(serialize = "StructureAreaPowerControl")]
    #[strum(
        props(
            name = "Area Power Control",
            desc = "An Area Power Control (APC) has three main functions: \nIts primary purpose is to regulate power flow, ensuring uninterrupted performance from devices and machinery, especially those with a fluctuating draw. \nAPCs also create sub-networks, as no devices on the far side of an APC are visible on the main network.\nLastly, an APC charges batteries, which can provide backup power to the sub-network in the case of an outage. Note that an APC requires a battery to stabilize power draw. It also has two variants, each allowing power to flow in one direction only.",
            value = "1999523701"
        )
    )]
    StructureAreaPowerControl = 1999523701i32,
    #[strum(serialize = "ItemGasFilterWaterL")]
    #[strum(props(name = "Heavy Filter (Water)", desc = "", value = "2004969680"))]
    ItemGasFilterWaterL = 2004969680i32,
    #[strum(serialize = "ItemDrill")]
    #[strum(
        props(
            name = "Hand Drill",
            desc = "The <link=ExMin><color=#0080FFFF>ExMin</color></link> Off-whirled Hand Drill has been a companion to <link=Stationeers><color=#0080FFFF>Stationeers</color></link> for decades. Essential for assembling and deconstructing various items and structures, regardless of gravity, pressure or temperature.",
            value = "2009673399"
        )
    )]
    ItemDrill = 2009673399i32,
    #[strum(serialize = "ItemFlagSmall")]
    #[strum(props(name = "Kit (Small Flag)", desc = "", value = "2011191088"))]
    ItemFlagSmall = 2011191088i32,
    #[strum(serialize = "ItemCookedRice")]
    #[strum(
        props(
            name = "Cooked Rice",
            desc = "A high-nutrient cooked food, which can be canned.",
            value = "2013539020"
        )
    )]
    ItemCookedRice = 2013539020i32,
    #[strum(serialize = "StructureRocketScanner")]
    #[strum(props(name = "Rocket Scanner", desc = "", value = "2014252591"))]
    StructureRocketScanner = 2014252591i32,
    #[strum(serialize = "ItemKitPoweredVent")]
    #[strum(props(name = "Kit (Powered Vent)", desc = "", value = "2015439334"))]
    ItemKitPoweredVent = 2015439334i32,
    #[strum(serialize = "CircuitboardSolarControl")]
    #[strum(
        props(
            name = "Solar Control",
            desc = "Adding a solar control board to a <link=ThingStructureConsole><color=green>Console</color></link> lets you manually control the horizontal and vertical angles of any connected <link=ThingStructureSolarPanel><color=green>Solar Panel</color></link>.",
            value = "2020180320"
        )
    )]
    CircuitboardSolarControl = 2020180320i32,
    #[strum(serialize = "StructurePipeRadiatorFlatLiquid")]
    #[strum(
        props(
            name = "Pipe Radiator Liquid",
            desc = "A liquid pipe mounted radiator optimized for radiating heat in vacuums.",
            value = "2024754523"
        )
    )]
    StructurePipeRadiatorFlatLiquid = 2024754523i32,
    #[strum(serialize = "StructureWallPaddingLightFitting")]
    #[strum(
        props(name = "Wall (Padding Light Fitting)", desc = "", value = "2024882687")
    )]
    StructureWallPaddingLightFitting = 2024882687i32,
    #[strum(serialize = "StructureReinforcedCompositeWindow")]
    #[strum(
        props(
            name = "Reinforced Window (Composite)",
            desc = "Enjoy vistas of even the most savage, alien landscapes with these heavy duty window frames, which are resistant to pressure differentials up to 1MPa.",
            value = "2027713511"
        )
    )]
    StructureReinforcedCompositeWindow = 2027713511i32,
    #[strum(serialize = "ItemKitRocketLiquidFuelTank")]
    #[strum(
        props(name = "Kit (Rocket Liquid Fuel Tank)", desc = "", value = "2032027950")
    )]
    ItemKitRocketLiquidFuelTank = 2032027950i32,
    #[strum(serialize = "StructureEngineMountTypeA1")]
    #[strum(props(name = "Engine Mount (Type A1)", desc = "", value = "2035781224"))]
    StructureEngineMountTypeA1 = 2035781224i32,
    #[strum(serialize = "ItemLiquidDrain")]
    #[strum(props(name = "Kit (Liquid Drain)", desc = "", value = "2036225202"))]
    ItemLiquidDrain = 2036225202i32,
    #[strum(serialize = "ItemLiquidTankStorage")]
    #[strum(
        props(
            name = "Kit (Liquid Canister Storage)",
            desc = "This kit produces a <link=ThingItemLiquidTankStorage><color=green>Kit (Liquid Canister Storage)</color></link> for refilling a <link=ThingItemLiquidCanisterEmpty><color=green>Liquid Canister</color></link>.",
            value = "2037427578"
        )
    )]
    ItemLiquidTankStorage = 2037427578i32,
    #[strum(serialize = "StructurePipeCrossJunction3")]
    #[strum(
        props(
            name = "Pipe (3-Way Junction)",
            desc = "You can upgrade this pipe to an <link=ThingStructureInsulatedPipeCrossJunction3><color=green>Insulated Pipe (3-Way Junction)</color></link> using an <link=ThingItemKitInsulatedPipe><color=green>Kit (Insulated Pipe)</color></link> and a <link=ThingItemWrench><color=green>Wrench</color></link>.",
            value = "2038427184"
        )
    )]
    StructurePipeCrossJunction3 = 2038427184i32,
    #[strum(serialize = "ItemPeaceLily")]
    #[strum(
        props(
            name = "Peace Lily",
            desc = "A fetching lily with greater resistance to cold temperatures.",
            value = "2042955224"
        )
    )]
    ItemPeaceLily = 2042955224i32,
    #[strum(serialize = "PortableSolarPanel")]
    #[strum(props(name = "Portable Solar Panel", desc = "", value = "2043318949"))]
    PortableSolarPanel = 2043318949i32,
    #[strum(serialize = "ItemMushroom")]
    #[strum(
        props(
            name = "Mushroom",
            desc = "A tasty food item. Unlike normal plants, it consumes <link=GasOxygen><color=#44AD83>Oxygen</color></link> and outputs <link=GasCarbonDioxide><color=#44AD83>Carbon Dioxide</color></link>. Mushrooms will only mature at a moderate rate in darkness, and prolonged light will kill it.",
            value = "2044798572"
        )
    )]
    ItemMushroom = 2044798572i32,
    #[strum(serialize = "StructureStairwellNoDoors")]
    #[strum(props(name = "Stairwell (No Doors)", desc = "", value = "2049879875"))]
    StructureStairwellNoDoors = 2049879875i32,
    #[strum(serialize = "StructureWindowShutter")]
    #[strum(
        props(
            name = "Window Shutter",
            desc = "For those special, private moments, a window that can be closed to prying eyes. \n      \nWhen closed, has the heat transfer characteristics of a basic wall. Requires power, and can be connected to logic systems.",
            value = "2056377335"
        )
    )]
    StructureWindowShutter = 2056377335i32,
    #[strum(serialize = "ItemKitHydroponicStation")]
    #[strum(props(name = "Kit (Hydroponic Station)", desc = "", value = "2057179799"))]
    ItemKitHydroponicStation = 2057179799i32,
    #[strum(serialize = "ItemCableCoilHeavy")]
    #[strum(
        props(
            name = "Cable Coil (Heavy)",
            desc = "Use heavy cable coil for power systems with large draws. Unlike <link=ThingStructureCableCoil><color=green><N:EN:StructureCableCoil></color></link>, which can only safely conduct 5kW, heavy cables can transmit up to 100kW.",
            value = "2060134443"
        )
    )]
    ItemCableCoilHeavy = 2060134443i32,
    #[strum(serialize = "StructureElevatorLevelIndustrial")]
    #[strum(props(name = "Elevator Level", desc = "", value = "2060648791"))]
    StructureElevatorLevelIndustrial = 2060648791i32,
    #[strum(serialize = "StructurePassiveLargeRadiatorGas")]
    #[strum(
        props(
            name = "Medium Convection Radiator",
            desc = "Has been replaced by <link=ThingStructureMediumConvectionRadiator><color=green>Medium Convection Radiator</color></link>.",
            value = "2066977095"
        )
    )]
    StructurePassiveLargeRadiatorGas = 2066977095i32,
    #[strum(serialize = "ItemKitInsulatedLiquidPipe")]
    #[strum(
        props(name = "Kit (Insulated Liquid Pipe)", desc = "", value = "2067655311")
    )]
    ItemKitInsulatedLiquidPipe = 2067655311i32,
    #[strum(serialize = "StructureLiquidPipeRadiator")]
    #[strum(
        props(
            name = "Liquid Pipe Convection Radiator",
            desc = "A simple heat exchanger, pipe radiators can be placed on pipes to shed or gain heat, depending on the temperature of the surrounding atmosphere. If the atmosphere is hotter, heat will be added to the liquid within the pipe network, and visa versa if colder. In a vacuum, heat will be radiated. \nThe speed of heat gain or loss will depend on the liquid in question. Adding multiple radiators will speed up heat transfer.",
            value = "2072805863"
        )
    )]
    StructureLiquidPipeRadiator = 2072805863i32,
    #[strum(serialize = "StructureLogicHashGen")]
    #[strum(props(name = "Logic Hash Generator", desc = "", value = "2077593121"))]
    StructureLogicHashGen = 2077593121i32,
    #[strum(serialize = "AccessCardWhite")]
    #[strum(props(name = "Access Card (White)", desc = "", value = "2079959157"))]
    AccessCardWhite = 2079959157i32,
    #[strum(serialize = "StructureCableStraightHBurnt")]
    #[strum(props(name = "Burnt Cable (Straight)", desc = "", value = "2085762089"))]
    StructureCableStraightHBurnt = 2085762089i32,
    #[strum(serialize = "StructureWallPaddedWindow")]
    #[strum(props(name = "Wall (Padded Window)", desc = "", value = "2087628940"))]
    StructureWallPaddedWindow = 2087628940i32,
    #[strum(serialize = "StructureLogicMirror")]
    #[strum(props(name = "Logic Mirror", desc = "", value = "2096189278"))]
    StructureLogicMirror = 2096189278i32,
    #[strum(serialize = "StructureWallFlatCornerTriangle")]
    #[strum(
        props(name = "Wall (Flat Corner Triangle)", desc = "", value = "2097419366")
    )]
    StructureWallFlatCornerTriangle = 2097419366i32,
    #[strum(serialize = "StructureBackLiquidPressureRegulator")]
    #[strum(
        props(
            name = "Liquid Back Volume Regulator",
            desc = "Regulates the volume ratio of liquid in the input Liquid pipe. This is expressed as percentage where 100 is totally full and 0 is empty.",
            value = "2099900163"
        )
    )]
    StructureBackLiquidPressureRegulator = 2099900163i32,
    #[strum(serialize = "StructureTankSmallFuel")]
    #[strum(props(name = "Small Tank (Fuel)", desc = "", value = "2102454415"))]
    StructureTankSmallFuel = 2102454415i32,
    #[strum(serialize = "ItemEmergencyWireCutters")]
    #[strum(props(name = "Emergency Wire Cutters", desc = "", value = "2102803952"))]
    ItemEmergencyWireCutters = 2102803952i32,
    #[strum(serialize = "StructureGasMixer")]
    #[strum(
        props(
            name = "Gas Mixer",
            desc = "Indispensable for producing precise atmospheric ratios, this gas mixer blends two gases in proportions ranging anywhere from 0-100%.",
            value = "2104106366"
        )
    )]
    StructureGasMixer = 2104106366i32,
    #[strum(serialize = "StructureCompositeFloorGratingOpen")]
    #[strum(
        props(name = "Composite Floor Grating Open", desc = "", value = "2109695912")
    )]
    StructureCompositeFloorGratingOpen = 2109695912i32,
    #[strum(serialize = "ItemRocketMiningDrillHead")]
    #[strum(
        props(
            name = "Mining-Drill Head (Basic)",
            desc = "Replaceable drill head for <link=ThingStructureRocketMiner><color=green>Rocket Miner</color></link>",
            value = "2109945337"
        )
    )]
    ItemRocketMiningDrillHead = 2109945337i32,
    #[strum(serialize = "ItemSugar")]
    #[strum(props(name = "Sugar", desc = "", value = "2111910840"))]
    ItemSugar = 2111910840i32,
    #[strum(serialize = "DynamicMKIILiquidCanisterEmpty")]
    #[strum(
        props(
            name = "Portable Liquid Tank Mk II",
            desc = "An empty, insulated liquid <link=SlotGasCanister><color=orange>Gas Canister</color></link>.",
            value = "2130739600"
        )
    )]
    DynamicMkiiLiquidCanisterEmpty = 2130739600i32,
    #[strum(serialize = "ItemSpaceOre")]
    #[strum(
        props(
            name = "Dirty Ore",
            desc = "Ore mined from asteroids via the <link=ThingStructureRocketMiner><color=green>Rocket Miner</color></link> which then must be processed in the <link=ThingStructureCentrifuge><color=green>Centrifuge</color></link>, or <link=ThingStructureCombustionCentrifuge><color=green>Combustion Centrifuge</color></link> to produce smeltable ores.",
            value = "2131916219"
        )
    )]
    ItemSpaceOre = 2131916219i32,
    #[strum(serialize = "ItemKitStandardChute")]
    #[strum(props(name = "Kit (Powered Chutes)", desc = "", value = "2133035682"))]
    ItemKitStandardChute = 2133035682i32,
    #[strum(serialize = "StructureInsulatedPipeStraight")]
    #[strum(
        props(
            name = "Insulated Pipe (Straight)",
            desc = "Insulated pipes greatly reduce heat loss from gases stored in them.",
            value = "2134172356"
        )
    )]
    StructureInsulatedPipeStraight = 2134172356i32,
    #[strum(serialize = "ItemLeadIngot")]
    #[strum(props(name = "Ingot (Lead)", desc = "", value = "2134647745"))]
    ItemLeadIngot = 2134647745i32,
    #[strum(serialize = "ItemGasCanisterNitrogen")]
    #[strum(props(name = "Canister (Nitrogen)", desc = "", value = "2145068424"))]
    ItemGasCanisterNitrogen = 2145068424i32,
}
impl TryFrom<f64> for StationpediaPrefab {
    type Error = super::ParseError;
    fn try_from(
        value: f64,
    ) -> Result<Self, <StationpediaPrefab as TryFrom<f64>>::Error> {
        use strum::IntoEnumIterator;
        if let Some(enm) = StationpediaPrefab::iter()
            .find(|enm| (f64::from(*enm as i32) - value).abs() < f64::EPSILON)
        {
            Ok(enm)
        } else {
            Err(super::ParseError {
                enm: value.to_string(),
            })
        }
    }
}

"use strict";

var oop = require("ace-code/src/lib/oop");
var TextHighlightRules = require("ace-code/src/mode/text_highlight_rules").TextHighlightRules;

var IC10HighlightRules = function() {
    // regexp must not have capturing parentheses. Use (?:) instead.
    // regexps are ordered -> the first match is used

    var ops = (
        "abs|acos|add|alias|and|asin|atan|atan2|bap|bapal|bapz|bapzal|bdns|"+
        "bdnsal|bdse|bdseal|beq|beqal|beqz|beqzal|bge|bgeal|bgez|bgezal|bgt|"+
        "bgtal|bgtz|bgtzal|ble|bleal|blez|blezal|blt|bltal|bltz|bltzal|bna|"+
        "bnaal|bnan|bnaz|bnazal|bne|bneal|bnez|bnezal|brap|brapz|brdns|brdse|"+
        "breq|breqz|brge|brgez|brgt|brgtz|brle|brlez|brlt|brltz|brna|brnan|"+
        "brnaz|brne|brnez|ceil|cos|define|div|exp|floor|get|getd|hcf|j|jal|jr|"+
        "l|label|lb|lbn|lbns|lbs|ld|log|lr|ls|max|min|mod|move|mul|nor|not|or|"+
        "peek|poke|pop|push|put|putd|rand|round|s|sap|sapz|sb|sbn|sbs|sd|sdns|"+
        "sdse|select|seq|seqz|sge|sgez|sgt|sgtz|sin|sla|sle|sleep|slez|sll|"+
        "slt|sltz|sna|snan|snanz|snaz|sne|snez|sqrt|sra|srl|ss|sub|tan|trunc|"+
        "xor|yield"
    );
    
    var enums = (
        "LogicType\\.Power|LogicType\\.Open|LogicType\\.Mode|LogicType\\.Error|"+
        "LogicType\\.Pressure|LogicType\\.Temperature|LogicType\\.PressureExternal|"+
        "LogicType\\.PressureInternal|LogicType\\.Activate|LogicType\\.Lock|"+
        "LogicType\\.Charge|LogicType\\.Setting|LogicType\\.Reagents|"+
        "LogicType\\.RatioOxygen|LogicType\\.RatioCarbonDioxide|"+
        "LogicType\\.RatioNitrogen|LogicType\\.RatioPollutant|"+
        "LogicType\\.RatioVolatiles|LogicType\\.RatioWater|LogicType\\.Horizontal|"+
        "LogicType\\.Vertical|LogicType\\.SolarAngle|LogicType\\.Maximum|"+
        "LogicType\\.Ratio|LogicType\\.PowerPotential|LogicType\\.PowerActual|"+
        "LogicType\\.Quantity|LogicType\\.On|LogicType\\.RequiredPower|"+
        "LogicType\\.HorizontalRatio|LogicType\\.VerticalRatio|"+
        "LogicType\\.PowerRequired|LogicType\\.Idle|LogicType\\.Color|"+
        "LogicType\\.ElevatorSpeed|LogicType\\.ElevatorLevel|"+
        "LogicType\\.RecipeHash|LogicType\\.RequestHash|LogicType\\.CompletionRatio|"+
        "LogicType\\.ClearMemory|LogicType\\.ExportCount|LogicType\\.ImportCount|"+
        "LogicType\\.PowerGeneration|LogicType\\.TotalMoles|LogicType\\.Volume|"+
        "LogicType\\.Plant|LogicType\\.Harvest|LogicType\\.Output|"+
        "LogicType\\.PressureSetting|LogicType\\.TemperatureSetting|"+
        "LogicType\\.TemperatureExternal|LogicType\\.Filtration|"+
        "LogicType\\.AirRelease|LogicType\\.PositionX|LogicType\\.PositionZ|"+
        "LogicType\\.VelocityMagnitude|LogicType\\.VelocityRelativeX|"+
        "LogicType\\.VelocityRelativeY|LogicType\\.VelocityRelativeZ|"+
        "LogicType\\.RatioNitrousOxide|LogicType\\.PrefabHash|"+
        "LogicType\\.ForceWrite|LogicType\\.SignalStrength|LogicType\\.SignalID|"+
        "LogicType\\.TargetX|LogicType\\.TargetY|LogicType\\.TargetZ|"+
        "LogicType\\.SettingInput|LogicType\\.SettingOutput|"+
        "LogicType\\.CurrentResearchPodType|LogicType\\.ManualResearchRequiredPod|"+
        "LogicType\\.MineablesInVicinity|LogicType\\.MineablesInQueue|"+
        "LogicType\\.NextWeatherEventTime|LogicType\\.Combustion|LogicType\\.Fuel|"+
        "LogicType\\.ReturnFuelCost|LogicType\\.CollectableGoods|LogicType\\.Time|"+
        "LogicType\\.Bpm|LogicType\\.EnvironmentEfficiency|"+
        "LogicType\\.WorkingGasEfficiency|LogicType\\.PressureInput|"+
        "LogicType\\.TemperatureInput|LogicType\\.RatioOxygenInput|"+
        "LogicType\\.RatioCarbonDioxideInput|LogicType\\.RatioNitrogenInput|"+
        "LogicType\\.RatioPollutantInput|LogicType\\.RatioVolatilesInput|"+
        "LogicType\\.RatioWaterInput|LogicType\\.RatioNitrousOxideInput|"+
        "LogicType\\.TotalMolesInput|LogicType\\.PressureInput2|"+
        "LogicType\\.TemperatureInput2|LogicType\\.RatioOxygenInput2|"+
        "LogicType\\.RatioCarbonDioxideInput2|"+
        "LogicType\\.RatioLiquidCarbonDioxide|LogicType\\.RatioNitrogenInput2|"+
        "LogicType\\.RatioPollutantInput2|LogicType\\.RatioVolatilesInput2|"+
        "LogicType\\.RatioWaterInput2|LogicType\\.RatioNitrousOxideInput2|"+
        "LogicType\\.TotalMolesInput2|LogicType\\.PressureOutput|"+
        "LogicType\\.TemperatureOutput|LogicType\\.RatioOxygenOutput|"+
        "LogicType\\.RatioCarbonDioxideOutput|LogicType\\.RatioNitrogenOutput|"+
        "LogicType\\.RatioPollutantOutput|LogicType\\.RatioVolatilesOutput|"+
        "LogicType\\.RatioWaterOutput|LogicType\\.RatioNitrousOxideOutput|"+
        "LogicType\\.TotalMolesOutput|LogicType\\.PressureOutput2|"+
        "LogicType\\.TemperatureOutput2|LogicType\\.RatioOxygenOutput2|"+
        "LogicType\\.RatioCarbonDioxideOutput2|LogicType\\.RatioNitrogenOutput2|"+
        "LogicType\\.RatioPollutantOutput2|LogicType\\.RatioVolatilesOutput2|"+
        "LogicType\\.RatioWaterOutput2|LogicType\\.RatioNitrousOxideOutput2|"+
        "LogicType\\.TotalMolesOutput2|LogicType\\.CombustionInput|"+
        "LogicType\\.CombustionInput2|LogicType\\.CombustionOutput|"+
        "LogicType\\.CombustionOutput2|"+
        "LogicType\\.OperationalTemperatureEfficiency|"+
        "LogicType\\.TemperatureDifferentialEfficiency|"+
        "LogicType\\.PressureEfficiency|LogicType\\.CombustionLimiter|"+
        "LogicType\\.Throttle|LogicType\\.Rpm|LogicType\\.Stress|"+
        "LogicType\\.InterrogationProgress|LogicType\\.TargetPadIndex|"+
        "LogicType\\.SizeX|LogicType\\.SizeY|LogicType\\.SizeZ|"+
        "LogicType\\.MinimumWattsToContact|LogicType\\.WattsReachingContact|"+
        "LogicType\\.Channel0|LogicType\\.Channel1|LogicType\\.Channel2|"+
        "LogicType\\.Channel3|LogicType\\.Channel4|LogicType\\.Channel5|"+
        "LogicType\\.Channel6|LogicType\\.LineNumber|LogicType\\.Flush|"+
        "LogicType\\.Channel7|LogicType\\.SoundAlert|LogicType\\.SolarIrradiance|"+
        "LogicType\\.RatioLiquidNitrogen|LogicType\\.RatioLiquidNitrogenInput|"+
        "LogicType\\.RatioLiquidNitrogenInput2|"+
        "LogicType\\.RatioLiquidNitrogenOutput|"+
        "LogicType\\.RatioLiquidNitrogenOutput2|"+
        "LogicType\\.VolumeOfLiquid|LogicType\\.RatioLiquidOxygen|"+
        "LogicType\\.RatioLiquidOxygenInput|LogicType\\.RatioLiquidOxygenInput2|"+
        "LogicType\\.RatioLiquidOxygenOutput|"+
        "LogicType\\.RatioLiquidOxygenOutput2|LogicType\\.RatioLiquidVolatiles|"+
        "LogicType\\.RatioLiquidVolatilesInput|"+
        "LogicType\\.RatioLiquidVolatilesInput2|"+
        "LogicType\\.RatioLiquidVolatilesOutput|"+
        "LogicType\\.RatioLiquidVolatilesOutput2|LogicType\\.RatioSteam|"+
        "LogicType\\.RatioSteamInput|LogicType\\.RatioSteamInput2|"+
        "LogicType\\.RatioSteamOutput|LogicType\\.RatioSteamOutput2|"+
        "LogicType\\.ContactTypeId|LogicType\\.RatioLiquidCarbonDioxideInput|"+
        "LogicType\\.RatioLiquidCarbonDioxideInput2|"+
        "LogicType\\.RatioLiquidCarbonDioxideOutput|"+
        "LogicType\\.RatioLiquidCarbonDioxideOutput2|"+
        "LogicType\\.RatioLiquidPollutant|LogicType\\.RatioLiquidPollutantInput|"+
        "LogicType\\.RatioLiquidPollutantInput2|"+
        "LogicType\\.RatioLiquidPollutantOutput|"+
        "LogicType\\.RatioLiquidPollutantOutput2|"+
        "LogicType\\.RatioLiquidNitrousOxide|"+
        "LogicType\\.RatioLiquidNitrousOxideInput|"+
        "LogicType\\.RatioLiquidNitrousOxideInput2|"+
        "LogicType\\.RatioLiquidNitrousOxideOutput|"+
        "LogicType\\.RatioLiquidNitrousOxideOutput2|"+
        "LogicType\\.Progress|LogicType\\.DestinationCode|"+
        "LogicType\\.Acceleration|LogicType\\.ReferenceId|"+
        "LogicType\\.AutoShutOff|LogicType\\.Mass|LogicType\\.DryMass|"+
        "LogicType\\.Thrust|LogicType\\.Weight|LogicType\\.ThrustToWeight|"+
        "LogicType\\.TimeToDestination|LogicType\\.BurnTimeRemaining|"+
        "LogicType\\.AutoLand|LogicType\\.ForwardX|LogicType\\.ForwardY|"+
        "LogicType\\.ForwardZ|LogicType\\.Orientation|LogicType\\.VelocityX|"+
        "LogicType\\.VelocityY|LogicType\\.VelocityZ|LogicType\\.PassedMoles|"+
        "LogicType\\.ExhaustVelocity|LogicType\\.FlightControlRule|"+
        "LogicType\\.ReEntryAltitude|LogicType\\.Apex|LogicType\\.EntityState|"+
        "LogicType\\.DrillCondition|LogicType\\.Index|LogicType\\.CelestialHash|"+
        "LogicType\\.AlignmentError|LogicType\\.DistanceAu|LogicType\\.OrbitPeriod|"+
        "LogicType\\.Inclination|LogicType\\.Eccentricity|LogicType\\.SemiMajorAxis|"+
        "LogicType\\.DistanceKm|LogicType\\.CelestialParentHash|"+
        "LogicType\\.TrueAnomaly|"+
        "LogicSlotType\\.None|LogicSlotType\\.Occupied|LogicSlotType\\.OccupantHash|"+
        "LogicSlotType\\.Quantity|LogicSlotType\\.Damage|LogicSlotType\\.Efficiency|"+
        "LogicSlotType\\.Health|LogicSlotType\\.Growth|LogicSlotType\\.Pressure|"+
        "LogicSlotType\\.Temperature|LogicSlotType\\.Charge|"+
        "LogicSlotType\\.ChargeRatio|LogicSlotType\\.Class|"+
        "LogicSlotType\\.PressureWaste|LogicSlotType\\.PressureAir|"+
        "LogicSlotType\\.MaxQuantity|LogicSlotType\\.Mature|"+
        "LogicSlotType\\.PrefabHash|LogicSlotType\\.Seeding|"+
        "LogicSlotType\\.LineNumber|LogicSlotType\\.Volume|"+
        "LogicSlotType\\.Open|LogicSlotType\\.On|LogicSlotType\\.Lock|"+
        "LogicSlotType\\.SortingClass|LogicSlotType\\.FilterType|"+
        "LogicSlotType\\.ReferenceId|"+
        "SlotClass\\.None|SlotClass\\.Helmet|SlotClass\\.Suit|SlotClass\\.Back|"+
        "SlotClass\\.GasFilter|SlotClass\\.Motherboard|SlotClass\\.Circuitboard|"+
        "SlotClass\\.DataDisk|SlotClass\\.Organ|SlotClass\\.Ore|SlotClass\\.Plant|"+
        "SlotClass\\.Uniform|SlotClass\\.Battery|SlotClass\\.Egg|SlotClass\\.Belt|"+
        "SlotClass\\.Tool|SlotClass\\.Appliance|SlotClass\\.Ingot|SlotClass\\.Torpedo|"+
        "SlotClass\\.Cartridge|SlotClass\\.AccessCard|SlotClass\\.Magazine|"+
        "SlotClass\\.Circuit|SlotClass\\.Bottle|SlotClass\\.ProgrammableChip|"+
        "SlotClass\\.Glasses|SlotClass\\.CreditCard|SlotClass\\.DirtCanister|"+
        "SlotClass\\.SensorProcessingUnit|SlotClass\\.LiquidCanister|"+
        "SlotClass\\.LiquidBottle|SlotClass\\.Wreckage|SlotClass\\.SoundCartridge|"+
        "SlotClass\\.DrillHead|SlotClass\\.ScanningHead|SlotClass\\.Flare|"+
        "SlotClass\\.Blocked|GasType\\.Undefined|GasType\\.Oxygen|GasType\\.Nitrogen|"+
        "GasType\\.CarbonDioxide|GasType\\.Volatiles|GasType\\.Pollutant|"+
        "GasType\\.Water|GasType\\.NitrousOxide|GasType\\.LiquidNitrogen|"+
        "GasType\\.LiquidVolatiles|GasType\\.Steam|GasType\\.LiquidCarbonDioxide|"+
        "GasType\\.LiquidPollutant|GasType\\.LiquidNitrousOxide|"+
        "Equals|Greater|Less|NotEquals|"+
        "AirCon\\.Cold|AirCon\\.Hot|AirControl\\.None|AirControl\\.Offline|"+
        "AirControl\\.Pressure|AirControl\\.Draught|"+
        "Color\\.Blue|Color\\.Gray|Color\\.Green|Color\\.Orange|Color\\.Red|Color\\.Yellow|"+
        "Color\\.White|Color\\.Black|Color\\.Brown|Color\\.Khaki|Color\\.Pink|"+
        "Color\\.Purple|"+
        "DaylightSensorMode\\.Default|DaylightSensorMode\\.Horizontal|"+
        "DaylightSensorMode\\.Vertical|"+
        "ElevatorMode\\.Stationary|ElevatorMode\\.Upward|ElevatorMode\\.Downward|"+
        "EntityState\\.Alive|EntityState\\.Dead|EntityState\\.Unconscious"+
        "|EntityState\\.Decay|"+
        "PowerMode\\.Idle|PowerMode\\.Discharged|PowerMode\\.Discharging|"+
        "PowerMode\\.Charging|PowerMode\\.Charged|"+
        "RobotMode\\.None|RobotMode\\.Follow|RobotMode\\.MoveToTarget|"+
        "RobotMode\\.Roam|RobotMode\\.Unload|RobotMode\\.PathToTarget|"+
        "RobotMode\\.StorageFull|"+
        "SortingClass\\.Default|SortingClass\\.Kits|SortingClass\\.Tools|"+
        "SortingClass\\.Resources|SortingClass\\.Food|SortingClass\\.Clothing|"+
        "SortingClass\\.Appliances|SortingClass\\.Atmospherics|"+
        "SortingClass\\.Storage|SortingClass\\.Ores|SortingClass\\.Ices|"+
        "TransmitterMode\\.Passive|TransmitterMode\\.Active|"+
        "Vent\\.Outward|Vent\\.Inward"
    );
    
    var logictypes = (
        "Acceleration|Activate|AirRelease|AlignmentError|Apex|AutoLand|"+
        "AutoShutOff|Average|Bpm|BurnTimeRemaining|Bypass|CelestialHash|"+
        "CelestialParentHash|Channel|Channel0|Channel1|Channel2|Channel3|"+
        "Channel4|Channel5|Channel6|Channel7|Charge|ChargeRatio|Class|"+
        "ClearMemory|CollectableGoods|Color|Combustion|CombustionInput|"+
        "CombustionInput2|CombustionLimiter|CombustionOutput|CombustionOutput2|"+
        "CompletionRatio|ContactTypeId|Contents|CurrentResearchPodType|Damage|"+
        "DestinationCode|DistanceAu|DistanceKm|DrillCondition|DryMass|"+
        "Eccentricity|Efficiency|ElevatorLevel|ElevatorSpeed|EntityState|"+
        "EnvironmentEfficiency|Error|ExhaustVelocity|ExportCount|"+
        "ExportQuantity|ExportSlotHash|ExportSlotOccupant|FilterType|"+
        "Filtration|FlightControlRule|Flush|ForceWrite|ForwardX|ForwardY|"+
        "ForwardZ|Fuel|Growth|Harvest|Health|Horizontal|HorizontalRatio|"+
        "Idle|ImportCount|ImportQuantity|ImportSlotHash|ImportSlotOccupant|"+
        "Inclination|Index|InterrogationProgress|LineNumber|Lock|"+
        "ManualResearchRequiredPod|Mass|Mature|MaxQuantity|Maximum|"+
        "MinWattsToContact|MineablesInQueue|MineablesInVicinity|Minimum|"+
        "MinimumWattsToContact|Mode|NextWeatherEventTime|None|OccupantHash|"+
        "Occupied|On|Open|OperationalTemperatureEfficiency|OrbitPeriod|"+
        "Orientation|Output|OverShootTarget|PassedMoles|Plant|"+
        "PlantEfficiency1|PlantEfficiency2|PlantEfficiency3|PlantEfficiency4|"+
        "PlantGrowth1|PlantGrowth2|PlantGrowth3|PlantGrowth4|PlantHash1|"+
        "PlantHash2|PlantHash3|PlantHash4|PlantHealth1|PlantHealth2|"+
        "PlantHealth3|PlantHealth4|PositionX|PositionY|PositionZ|Power|"+
        "PowerActual|PowerGeneration|PowerPotential|PowerRequired|"+
        "PrefabHash|Pressure|PressureAir|PressureEfficiency|PressureExternal|"+
        "PressureInput|PressureInput2|PressureInternal|PressureOutput|"+
        "PressureOutput2|PressureSetting|PressureWaste|Progress|Quantity|"+
        "Ratio|RatioCarbonDioxide|RatioCarbonDioxideInput|"+
        "RatioCarbonDioxideInput2|RatioCarbonDioxideOutput|"+
        "RatioCarbonDioxideOutput2|RatioLiquidCarbonDioxide|"+
        "RatioLiquidCarbonDioxideInput|RatioLiquidCarbonDioxideInput2|"+
        "RatioLiquidCarbonDioxideOutput|RatioLiquidCarbonDioxideOutput2|"+
        "RatioLiquidNitrogen|RatioLiquidNitrogenInput|"+
        "RatioLiquidNitrogenInput2|RatioLiquidNitrogenOutput|"+
        "RatioLiquidNitrogenOutput2|RatioLiquidNitrousOxide|"+
        "RatioLiquidNitrousOxideInput|RatioLiquidNitrousOxideInput2|"+
        "RatioLiquidNitrousOxideOutput|RatioLiquidNitrousOxideOutput2|"+
        "RatioLiquidOxygen|RatioLiquidOxygenInput|RatioLiquidOxygenInput2|"+
        "RatioLiquidOxygenOutput|RatioLiquidOxygenOutput2|"+
        "RatioLiquidPollutant|RatioLiquidPollutantInput|"+
        "RatioLiquidPollutantInput2|RatioLiquidPollutantOutput|"+
        "RatioLiquidPollutantOutput2|RatioLiquidVolatiles|"+
        "RatioLiquidVolatilesInput|RatioLiquidVolatilesInput2|"+
        "RatioLiquidVolatilesOutput|RatioLiquidVolatilesOutput2|"+
        "RatioNitrogen|RatioNitrogenInput|RatioNitrogenInput2|"+
        "RatioNitrogenOutput|RatioNitrogenOutput2|RatioNitrousOxide|"+
        "RatioNitrousOxideInput|RatioNitrousOxideInput2|"+
        "RatioNitrousOxideOutput|RatioNitrousOxideOutput2|RatioOxygen|"+
        "RatioOxygenInput|RatioOxygenInput2|RatioOxygenOutput|"+
        "RatioOxygenOutput2|RatioPollutant|RatioPollutantInput|"+
        "RatioPollutantInput2|RatioPollutantOutput|RatioPollutantOutput2|"+
        "RatioSteam|RatioSteamInput|RatioSteamInput2|RatioSteamOutput|"+
        "RatioSteamOutput2|RatioVolatiles|RatioVolatilesInput|"+
        "RatioVolatilesInput2|RatioVolatilesOutput|RatioVolatilesOutput2|"+
        "RatioWater|RatioWaterInput|RatioWaterInput2|RatioWaterOutput|"+
        "RatioWaterOutput2|ReEntryAltitude|Reagents|Recipe|RecipeHash|"+
        "ReferenceId|RequestHash|Required|RequiredPower|ReturnFuelCost|Rpm|"+
        "Seeding|SemiMajorAxis|Setting|SettingInput|SettingInputHash|"+
        "SettingOutput|SettingOutputHash|SignalID|SignalStrength|SizeX|SizeY|"+
        "SizeZ|SolarAngle|SolarConstant|SolarIrradiance|SortingClass|"+
        "SoundAlert|Stress|Sum|TargetPadIndex|TargetX|TargetY|TargetZ|"+
        "Temperature|TemperatureDifferentialEfficiency|TemperatureExternal|"+
        "TemperatureInput|TemperatureInput2|TemperatureOutput|"+
        "TemperatureOutput2|TemperatureSetting|Throttle|Thrust|ThrustToWeight|"+
        "Time|TimeToDestination|TotalMoles|TotalMolesInput|TotalMolesInput2|"+
        "TotalMolesOutput|TotalMolesOutput2|TrueAnomaly|Unknown|"+
        "VelocityMagnitude|VelocityRelativeX|VelocityRelativeY|"+
        "VelocityRelativeZ|VelocityX|VelocityY|VelocityZ|Vertical|"+
        "VerticalRatio|Volume|VolumeOfLiquid|WattsReachingContact|Weight"+
        "|WorkingGasEfficiency"
    );
    
    var constants = ("nan|pinf|ninf|pi|deg2rad|rad2deg|epsilon")

    this.$rules = {
        start: [{
            token : "comment.line.number-sign",
            regex: /#.*$/,
        }, {
            token: ["text", "keyword", "text", "entity.name.tag"],
            regex: /^(\s*)(j(?:al)?)(\s+)([a-zA-Z_.][a-zA-Z0-9_.]*)/,
            comment: "absolute jumps | branches with a label"
        }, {
            token: "keyword",
            regex: "^\\s*(?:" + ops + ")\\b",
        },  {
            token: "entity.name.tag",
            regex: /^\s*[a-zA-Z_.][a-zA-Z0-9_.]*\b:/,
            comment: "Labels at line start: begin_repeat: add ..."
        }, {
            token: "variable.parameter",
            regex: /\b(?:sp|r(?:a|r*)(?:[0-9]|1[0-7]))\b/,
        }, {
            token: "variable.language",
            regex: /\b(?:d(?:b|[0-5]|r*(?:[0-9]|1[0-7]))(?::[0-9]+)?)\b/,
        }, {
            token: "support.type",
            regex: "\\b(?:" + logictypes + ")\\b",
        }, {
            token: "constant.language",
            regex: "\\b(?:" + constants + ")\\b",
        }, {
            token: "variable.language",
            regex: "\\b(?:" + enums  + ")\\b",
        }, {
            token: "constant.numeric.ic10",
            regex: /\-?[0-9]+(?:\.[0-9]+)?|\$[a-fA-F0-9_]+|%[01_]+/,
            comment: "Numbers like 12, -3.0, 55, $3F, %1000"
        }, {
            token: ["support.function", "paren.lparen", "string.quoted", "paren.rparen"],
            regex: /\b(HASH)(\()(\".*\")(\))/,
        }, {
            token: "entity.name",
            regex: /\b[a-zA-Z_.][a-zA-Z0-9_.]*\b/,
        }] 
    };

    this.normalizeRules();
};

IC10HighlightRules.metaData = {
    fileTypes: ["ic10"],
    name: "ic10",
    scopeName: "source.ic10"
};


oop.inherits(IC10HighlightRules, TextHighlightRules);

exports.IC10HighlightRules = IC10HighlightRules;

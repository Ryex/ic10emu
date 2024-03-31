

var ops = (
    "abs|acos|add|alias|and|asin|atan|atan2|bap|bapal|bapz|bapzal|bdns|" +
    "bdnsal|bdse|bdseal|beq|beqal|beqz|beqzal|bge|bgeal|bgez|bgezal|bgt|" +
    "bgtal|bgtz|bgtzal|ble|bleal|blez|blezal|blt|bltal|bltz|bltzal|bna|" +
    "bnaal|bnan|bnaz|bnazal|bne|bneal|bnez|bnezal|brap|brapz|brdns|brdse|" +
    "breq|breqz|brge|brgez|brgt|brgtz|brle|brlez|brlt|brltz|brna|brnan|" +
    "brnaz|brne|brnez|ceil|cos|define|div|exp|floor|get|getd|hcf|j|jal|jr|" +
    "l|label|lb|lbn|lbns|lbs|ld|log|lr|ls|max|min|mod|move|mul|nor|not|or|" +
    "peek|poke|pop|push|put|putd|rand|round|s|sap|sapz|sb|sbn|sbs|sd|sdns|" +
    "sdse|select|seq|seqz|sge|sgez|sgt|sgtz|sin|sla|sle|sleep|slez|sll|" +
    "slt|sltz|sna|snan|snanz|snaz|sne|snez|sqrt|sra|srl|ss|sub|tan|trunc|" +
    "xor|yield"
);

var enums = (
    "AirCon\\.Cold|AirCon\\.Hot|AirControl\\.Draught|AirControl\\.None|" +
    "AirControl\\.Offline|AirControl\\.Pressure|Color\\.Black|Color\\.Blue|" +
    "Color\\.Brown|Color\\.Gray|Color\\.Green|Color\\.Khaki|Color\\.Orange|" +
    "Color\\.Pink|Color\\.Purple|Color\\.Red|Color\\.White|Color\\.Yellow|" +
    "DaylightSensorMode\\.Default|DaylightSensorMode\\.Horizontal|" +
    "DaylightSensorMode\\.Vertical|ElevatorMode\\.Downward|" +
    "ElevatorMode\\.Stationary|ElevatorMode\\.Upward|EntityState\\.Alive|" +
    "EntityState\\.Dead|EntityState\\.Decay|EntityState\\.Unconscious|" +
    "Equals|GasType\\.CarbonDioxide|GasType\\.Hydrogen|GasType\\.LiquidCarbonDioxide|" +
    "GasType\\.LiquidHydrogen|GasType\\.LiquidNitrogen|GasType\\.LiquidNitrousOxide|" +
    "GasType\\.LiquidOxygen|GasType\\.LiquidPollutant|GasType\\.LiquidVolatiles|" +
    "GasType\\.Nitrogen|GasType\\.NitrousOxide|GasType\\.Oxygen|" +
    "GasType\\.Pollutant|GasType\\.PollutedWater|GasType\\.Steam|" +
    "GasType\\.Undefined|GasType\\.Volatiles|GasType\\.Water|" +
    "Greater|Less|LogicSlotType\\.Charge|LogicSlotType\\.ChargeRatio|" +
    "LogicSlotType\\.Class|LogicSlotType\\.Damage|LogicSlotType\\.Efficiency|" +
    "LogicSlotType\\.FilterType|LogicSlotType\\.Growth|LogicSlotType\\.Health|" +
    "LogicSlotType\\.LineNumber|LogicSlotType\\.Lock|LogicSlotType\\.Mature|" +
    "LogicSlotType\\.MaxQuantity|LogicSlotType\\.None|LogicSlotType\\.OccupantHash|" +
    "LogicSlotType\\.Occupied|LogicSlotType\\.On|LogicSlotType\\.Open|" +
    "LogicSlotType\\.PrefabHash|LogicSlotType\\.Pressure|" +
    "LogicSlotType\\.PressureAir|LogicSlotType\\.PressureWaste|" +
    "LogicSlotType\\.Quantity|LogicSlotType\\.ReferenceId|" +
    "LogicSlotType\\.Seeding|LogicSlotType\\.SortingClass|" +
    "LogicSlotType\\.Temperature|LogicSlotType\\.Volume|" +
    "LogicType\\.Acceleration|LogicType\\.Activate|LogicType\\.AirRelease|" +
    "LogicType\\.AlignmentError|LogicType\\.Apex|LogicType\\.AutoLand|" +
    "LogicType\\.AutoShutOff|LogicType\\.Bpm|LogicType\\.BurnTimeRemaining|" +
    "LogicType\\.CelestialHash|LogicType\\.CelestialParentHash|" +
    "LogicType\\.Channel1|LogicType\\.Channel1|LogicType\\.Channel2|" +
    "LogicType\\.Channel3|LogicType\\.Channel4|LogicType\\.Channel5|" +
    "LogicType\\.Channel6|LogicType\\.Channel7|LogicType\\.Charge|" +
    "LogicType\\.Chart|LogicType\\.ChartedNavPoints|LogicType\\.ClearMemory|" +
    "LogicType\\.CollectableGoods|LogicType\\.Color|LogicType\\.Combustion|" +
    "LogicType\\.CombustionInput|LogicType\\.CombustionInput2|" +
    "LogicType\\.CombustionLimiter|LogicType\\.CombustionOutput|" +
    "LogicType\\.CombustionOutput2|LogicType\\.CompletionRatio|" +
    "LogicType\\.ContactTypeId|LogicType\\.CurrentCode|LogicType\\.CurrentResearchPodType|" +
    "LogicType\\.Density|LogicType\\.DestinationCode|LogicType\\.Discover|" +
    "LogicType\\.DistanceAu|LogicType\\.DistanceKm|LogicType\\.DrillCondition|" +
    "LogicType\\.DryMass|LogicType\\.Eccentricity|LogicType\\.ElevatorLevel|" +
    "LogicType\\.ElevatorSpeed|LogicType\\.EntityState|LogicType\\.EnvironmentEfficiency|" +
    "LogicType\\.Error|LogicType\\.ExhaustVelocity|LogicType\\.ExportCount|" +
    "LogicType\\.ExportQuantity|LogicType\\.ExportSlotHash|" +
    "LogicType\\.ExportSlotOccupant|LogicType\\.Filtration|" +
    "LogicType\\.FlightControlRule|LogicType\\.Flush|LogicType\\.ForceWrite|" +
    "LogicType\\.ForwardX|LogicType\\.ForwardY|LogicType\\.ForwardZ|" +
    "LogicType\\.Fuel|LogicType\\.Harvest|LogicType\\.Horizontal|" +
    "LogicType\\.HorizontalRatio|LogicType\\.Idle|LogicType\\.ImportCount|" +
    "LogicType\\.ImportQuantity|LogicType\\.ImportSlotHash|" +
    "LogicType\\.ImportSlotOccupant|LogicType\\.Inclination|" +
    "LogicType\\.Index|LogicType\\.InterrogationProgress|" +
    "LogicType\\.LineNumber|LogicType\\.Lock|LogicType\\.ManualResearchRequiredPod|" +
    "LogicType\\.Mass|LogicType\\.Maximum|LogicType\\.MineablesInQueue|" +
    "LogicType\\.MineablesInVicinity|LogicType\\.MinedQuantity|" +
    "LogicType\\.MinimumWattsToContact|LogicType\\.Mode|" +
    "LogicType\\.NavPoints|LogicType\\.NextWeatherEventTime|" +
    "LogicType\\.None|LogicType\\.On|LogicType\\.Open|LogicType\\.OperationalTemperatureEfficiency|" +
    "LogicType\\.OrbitPeriod|LogicType\\.Orientation|LogicType\\.Output|" +
    "LogicType\\.PassedMoles|LogicType\\.Plant|LogicType\\.PlantEfficiency1|" +
    "LogicType\\.PlantEfficiency2|LogicType\\.PlantEfficiency3|" +
    "LogicType\\.PlantEfficiency4|LogicType\\.PlantGrowth1|" +
    "LogicType\\.PlantGrowth2|LogicType\\.PlantGrowth3|LogicType\\.PlantGrowth4|" +
    "LogicType\\.PlantHash1|LogicType\\.PlantHash2|LogicType\\.PlantHash3|" +
    "LogicType\\.PlantHash4|LogicType\\.PlantHealth1|LogicType\\.PlantHealth2|" +
    "LogicType\\.PlantHealth3|LogicType\\.PlantHealth4|LogicType\\.PositionX|" +
    "LogicType\\.PositionY|LogicType\\.PositionZ|LogicType\\.Power|" +
    "LogicType\\.PowerActual|LogicType\\.PowerGeneration|" +
    "LogicType\\.PowerPotential|LogicType\\.PowerRequired|" +
    "LogicType\\.PrefabHash|LogicType\\.Pressure|LogicType\\.PressureEfficiency|" +
    "LogicType\\.PressureExternal|LogicType\\.PressureInput|" +
    "LogicType\\.PressureInput2|LogicType\\.PressureInternal|" +
    "LogicType\\.PressureOutput|LogicType\\.PressureOutput2|" +
    "LogicType\\.PressureSetting|LogicType\\.Progress|LogicType\\.Quantity|" +
    "LogicType\\.Ratio|LogicType\\.RatioCarbonDioxide|LogicType\\.RatioCarbonDioxideInput|" +
    "LogicType\\.RatioCarbonDioxideInput2|LogicType\\.RatioCarbonDioxideOutput|" +
    "LogicType\\.RatioCarbonDioxideOutput2|LogicType\\.RatioHydrogen|" +
    "LogicType\\.RatioLiquidCarbonDioxide|LogicType\\.RatioLiquidCarbonDioxideInput|" +
    "LogicType\\.RatioLiquidCarbonDioxideInput2|LogicType\\.RatioLiquidCarbonDioxideOutput|" +
    "LogicType\\.RatioLiquidCarbonDioxideOutput2|LogicType\\.RatioLiquidHydrogen|" +
    "LogicType\\.RatioLiquidNitrogen|LogicType\\.RatioLiquidNitrogenInput|" +
    "LogicType\\.RatioLiquidNitrogenInput2|LogicType\\.RatioLiquidNitrogenOutput|" +
    "LogicType\\.RatioLiquidNitrogenOutput2|LogicType\\.RatioLiquidNitrousOxide|" +
    "LogicType\\.RatioLiquidNitrousOxideInput|LogicType\\.RatioLiquidNitrousOxideInput2|" +
    "LogicType\\.RatioLiquidNitrousOxideOutput|LogicType\\.RatioLiquidNitrousOxideOutput2|" +
    "LogicType\\.RatioLiquidOxygen|LogicType\\.RatioLiquidOxygenInput|" +
    "LogicType\\.RatioLiquidOxygenInput2|LogicType\\.RatioLiquidOxygenOutput|" +
    "LogicType\\.RatioLiquidOxygenOutput2|LogicType\\.RatioLiquidPollutant|" +
    "LogicType\\.RatioLiquidPollutantInput|LogicType\\.RatioLiquidPollutantInput2|" +
    "LogicType\\.RatioLiquidPollutantOutput|LogicType\\.RatioLiquidPollutantOutput2|" +
    "LogicType\\.RatioLiquidVolatiles|LogicType\\.RatioLiquidVolatilesInput|" +
    "LogicType\\.RatioLiquidVolatilesInput2|LogicType\\.RatioLiquidVolatilesOutput|" +
    "LogicType\\.RatioLiquidVolatilesOutput2|LogicType\\.RatioNitrogen|" +
    "LogicType\\.RatioNitrogenInput|LogicType\\.RatioNitrogenInput2|" +
    "LogicType\\.RatioNitrogenOutput|LogicType\\.RatioNitrogenOutput2|" +
    "LogicType\\.RatioNitrousOxide|LogicType\\.RatioNitrousOxideInput|" +
    "LogicType\\.RatioNitrousOxideInput2|LogicType\\.RatioNitrousOxideOutput|" +
    "LogicType\\.RatioNitrousOxideOutput2|LogicType\\.RatioOxygen|" +
    "LogicType\\.RatioOxygenInput|LogicType\\.RatioOxygenInput2|" +
    "LogicType\\.RatioOxygenOutput|LogicType\\.RatioOxygenOutput2|" +
    "LogicType\\.RatioPollutant|LogicType\\.RatioPollutantInput|" +
    "LogicType\\.RatioPollutantInput2|LogicType\\.RatioPollutantOutput|" +
    "LogicType\\.RatioPollutantOutput2|LogicType\\.RatioPollutedWater|" +
    "LogicType\\.RatioSteam|LogicType\\.RatioSteamInput|" +
    "LogicType\\.RatioSteamInput2|LogicType\\.RatioSteamOutput|" +
    "LogicType\\.RatioSteamOutput2|LogicType\\.RatioVolatiles|" +
    "LogicType\\.RatioVolatilesInput|LogicType\\.RatioVolatilesInput2|" +
    "LogicType\\.RatioVolatilesOutput|LogicType\\.RatioVolatilesOutput2|" +
    "LogicType\\.RatioWater|LogicType\\.RatioWaterInput|" +
    "LogicType\\.RatioWaterInput2|LogicType\\.RatioWaterOutput|" +
    "LogicType\\.RatioWaterOutput2|LogicType\\.ReEntryAltitude|" +
    "LogicType\\.Reagents|LogicType\\.RecipeHash|LogicType\\.ReferenceId|" +
    "LogicType\\.RequestHash|LogicType\\.RequiredPower|LogicType\\.ReturnFuelCost|" +
    "LogicType\\.Richness|LogicType\\.Rpm|LogicType\\.SemiMajorAxis|" +
    "LogicType\\.Setting|LogicType\\.SettingInput|LogicType\\.SettingOutput|" +
    "LogicType\\.SignalID|LogicType\\.SignalStrength|LogicType\\.Sites|" +
    "LogicType\\.Size|LogicType\\.SizeX|LogicType\\.SizeY|" +
    "LogicType\\.SizeZ|LogicType\\.SolarAngle|LogicType\\.SolarIrradiance|" +
    "LogicType\\.SoundAlert|LogicType\\.Stress|LogicType\\.Survey|" +
    "LogicType\\.TargetPadIndex|LogicType\\.TargetX|LogicType\\.TargetY|" +
    "LogicType\\.TargetZ|LogicType\\.Temperature|LogicType\\.TemperatureDifferentialEfficiency|" +
    "LogicType\\.TemperatureExternal|LogicType\\.TemperatureInput|" +
    "LogicType\\.TemperatureInput2|LogicType\\.TemperatureOutput|" +
    "LogicType\\.TemperatureOutput2|LogicType\\.TemperatureSetting|" +
    "LogicType\\.Throttle|LogicType\\.Thrust|LogicType\\.ThrustToWeight|" +
    "LogicType\\.Time|LogicType\\.TimeToDestination|LogicType\\.TotalMoles|" +
    "LogicType\\.TotalMolesInput|LogicType\\.TotalMolesInput2|" +
    "LogicType\\.TotalMolesOutput|LogicType\\.TotalMolesOutput2|" +
    "LogicType\\.TotalQuantity|LogicType\\.TrueAnomaly|LogicType\\.VelocityMagnitude|" +
    "LogicType\\.VelocityRelativeX|LogicType\\.VelocityRelativeY|" +
    "LogicType\\.VelocityRelativeZ|LogicType\\.VelocityX|" +
    "LogicType\\.VelocityY|LogicType\\.VelocityZ|LogicType\\.Vertical|" +
    "LogicType\\.VerticalRatio|LogicType\\.Volume|LogicType\\.VolumeOfLiquid|" +
    "LogicType\\.WattsReachingContact|LogicType\\.Weight|" +
    "LogicType\\.WorkingGasEfficiency|NotEquals|PowerMode\\.Charged|" +
    "PowerMode\\.Charging|PowerMode\\.Discharged|PowerMode\\.Discharging|" +
    "PowerMode\\.Idle|RobotMode\\.Follow|RobotMode\\.MoveToTarget|" +
    "RobotMode\\.None|RobotMode\\.PathToTarget|RobotMode\\.Roam|" +
    "RobotMode\\.StorageFull|RobotMode\\.Unload|SlotClass\\.AccessCard|" +
    "SlotClass\\.Appliance|SlotClass\\.Back|SlotClass\\.Battery|" +
    "SlotClass\\.Belt|SlotClass\\.Blocked|SlotClass\\.Bottle|" +
    "SlotClass\\.Cartridge|SlotClass\\.Circuit|SlotClass\\.Circuitboard|" +
    "SlotClass\\.CreditCard|SlotClass\\.DataDisk|SlotClass\\.DirtCanister|" +
    "SlotClass\\.DrillHead|SlotClass\\.Egg|SlotClass\\.Entity|" +
    "SlotClass\\.Flare|SlotClass\\.GasCanister|SlotClass\\.GasFilter|" +
    "SlotClass\\.Glasses|SlotClass\\.Helmet|SlotClass\\.Ingot|" +
    "SlotClass\\.LiquidBottle|SlotClass\\.LiquidCanister|" +
    "SlotClass\\.Magazine|SlotClass\\.Motherboard|SlotClass\\.None|" +
    "SlotClass\\.Ore|SlotClass\\.Organ|SlotClass\\.Plant|" +
    "SlotClass\\.ProgrammableChip|SlotClass\\.ScanningHead|" +
    "SlotClass\\.SensorProcessingUnit|SlotClass\\.SoundCartridge|" +
    "SlotClass\\.Suit|SlotClass\\.Tool|SlotClass\\.Torpedo|" +
    "SlotClass\\.Uniform|SlotClass\\.Wreckage|SortingClass\\.Appliances|" +
    "SortingClass\\.Atmospherics|SortingClass\\.Clothing|" +
    "SortingClass\\.Default|SortingClass\\.Food|SortingClass\\.Ices|" +
    "SortingClass\\.Kits|SortingClass\\.Ores|SortingClass\\.Resources|" +
    "SortingClass\\.Storage|SortingClass\\.Tools|Sound\\.AirlockCycling|" +
    "Sound\\.Alarm1|Sound\\.Alarm10|Sound\\.Alarm11|Sound\\.Alarm12|" +
    "Sound\\.Alarm2|Sound\\.Alarm3|Sound\\.Alarm4|Sound\\.Alarm5|" +
    "Sound\\.Alarm6|Sound\\.Alarm7|Sound\\.Alarm8|Sound\\.Alarm9|" +
    "Sound\\.Alert|Sound\\.Danger|Sound\\.Depressurising|" +
    "Sound\\.FireFireFire|Sound\\.Five|Sound\\.Floor|Sound\\.Four|" +
    "Sound\\.HaltWhoGoesThere|Sound\\.HighCarbonDioxide|" +
    "Sound\\.IntruderAlert|Sound\\.LiftOff|Sound\\.MalfunctionDetected|" +
    "Sound\\.Music1|Sound\\.Music2|Sound\\.Music3|Sound\\.None|" +
    "Sound\\.One|Sound\\.PollutantsDetected|Sound\\.PowerLow|" +
    "Sound\\.PressureHigh|Sound\\.PressureLow|Sound\\.Pressurising|" +
    "Sound\\.RocketLaunching|Sound\\.StormIncoming|Sound\\.SystemFailure|" +
    "Sound\\.TemperatureHigh|Sound\\.TemperatureLow|Sound\\.Three|" +
    "Sound\\.TraderIncoming|Sound\\.TraderLanded|Sound\\.Two|" +
    "Sound\\.Warning|Sound\\.Welcome|TransmitterMode\\.Active|" +
    "TransmitterMode\\.Passive|Vent\\.Inward|Vent\\.Outward");

var logictypes = (
    "Acceleration|Activate|AirRelease|AlignmentError|Apex|AutoLand|" +
    "AutoShutOff|Bpm|BurnTimeRemaining|CelestialHash|CelestialParentHash|" +
    "Channel0|Channel1|Channel2|Channel3|Channel4|Channel5|Channel6|" +
    "Channel7|Charge|Chart|ChartedNavPoints|ClearMemory|CollectableGoods|" +
    "Color|Combustion|CombustionInput|CombustionInput2|CombustionLimiter|" +
    "CombustionOutput|CombustionOutput2|CompletionRatio|ContactTypeId|" +
    "CurrentCode|CurrentResearchPodType|Density|DestinationCode|Discover|" +
    "DistanceAu|DistanceKm|DrillCondition|DryMass|Eccentricity|ElevatorLevel|" +
    "ElevatorSpeed|EntityState|EnvironmentEfficiency|Error|ExhaustVelocity|" +
    "ExportCount|ExportQuantity|ExportSlotHash|ExportSlotOccupant|" +
    "Filtration|FlightControlRule|Flush|ForceWrite|ForwardX|ForwardY|" +
    "ForwardZ|Fuel|Harvest|Horizontal|HorizontalRatio|Idle|ImportCount|" +
    "ImportQuantity|ImportSlotHash|ImportSlotOccupant|Inclination|" +
    "Index|InterrogationProgress|LineNumber|Lock|ManualResearchRequiredPod|" +
    "Mass|Maximum|MineablesInQueue|MineablesInVicinity|MinedQuantity|" +
    "MinimumWattsToContact|Mode|NavPoints|NextWeatherEventTime|None|" +
    "On|Open|OperationalTemperatureEfficiency|OrbitPeriod|Orientation|" +
    "Output|PassedMoles|Plant|PlantEfficiency1|PlantEfficiency2|PlantEfficiency3|" +
    "PlantEfficiency4|PlantGrowth1|PlantGrowth2|PlantGrowth3|PlantGrowth4|" +
    "PlantHash1|PlantHash2|PlantHash3|PlantHash4|PlantHealth1|PlantHealth2|" +
    "PlantHealth3|PlantHealth4|PositionX|PositionY|PositionZ|Power|" +
    "PowerActual|PowerGeneration|PowerPotential|PowerRequired|PrefabHash|" +
    "Pressure|PressureEfficiency|PressureExternal|PressureInput|PressureInput2|" +
    "PressureInternal|PressureOutput|PressureOutput2|PressureSetting|" +
    "Progress|Quantity|Ratio|RatioCarbonDioxide|RatioCarbonDioxideInput|" +
    "RatioCarbonDioxideInput2|RatioCarbonDioxideOutput|RatioCarbonDioxideOutput2|" +
    "RatioHydrogen|RatioLiquidCarbonDioxide|RatioLiquidCarbonDioxideInput|" +
    "RatioLiquidCarbonDioxideInput2|RatioLiquidCarbonDioxideOutput|" +
    "RatioLiquidCarbonDioxideOutput2|RatioLiquidHydrogen|RatioLiquidNitrogen|" +
    "RatioLiquidNitrogenInput|RatioLiquidNitrogenInput2|RatioLiquidNitrogenOutput|" +
    "RatioLiquidNitrogenOutput2|RatioLiquidNitrousOxide|RatioLiquidNitrousOxideInput|" +
    "RatioLiquidNitrousOxideInput2|RatioLiquidNitrousOxideOutput|RatioLiquidNitrousOxideOutput2|" +
    "RatioLiquidOxygen|RatioLiquidOxygenInput|RatioLiquidOxygenInput2|" +
    "RatioLiquidOxygenOutput|RatioLiquidOxygenOutput2|RatioLiquidPollutant|" +
    "RatioLiquidPollutantInput|RatioLiquidPollutantInput2|RatioLiquidPollutantOutput|" +
    "RatioLiquidPollutantOutput2|RatioLiquidVolatiles|RatioLiquidVolatilesInput|" +
    "RatioLiquidVolatilesInput2|RatioLiquidVolatilesOutput|RatioLiquidVolatilesOutput2|" +
    "RatioNitrogen|RatioNitrogenInput|RatioNitrogenInput2|RatioNitrogenOutput|" +
    "RatioNitrogenOutput2|RatioNitrousOxide|RatioNitrousOxideInput|" +
    "RatioNitrousOxideInput2|RatioNitrousOxideOutput|RatioNitrousOxideOutput2|" +
    "RatioOxygen|RatioOxygenInput|RatioOxygenInput2|RatioOxygenOutput|" +
    "RatioOxygenOutput2|RatioPollutant|RatioPollutantInput|RatioPollutantInput2|" +
    "RatioPollutantOutput|RatioPollutantOutput2|RatioPollutedWater|" +
    "RatioSteam|RatioSteamInput|RatioSteamInput2|RatioSteamOutput|" +
    "RatioSteamOutput2|RatioVolatiles|RatioVolatilesInput|RatioVolatilesInput2|" +
    "RatioVolatilesOutput|RatioVolatilesOutput2|RatioWater|RatioWaterInput|" +
    "RatioWaterInput2|RatioWaterOutput|RatioWaterOutput2|ReEntryAltitude|" +
    "Reagents|RecipeHash|ReferenceId|RequestHash|RequiredPower|ReturnFuelCost|" +
    "Richness|Rpm|SemiMajorAxis|Setting|SettingInput|SettingOutput|" +
    "SignalID|SignalStrength|Sites|Size|SizeX|SizeY|SizeZ|SolarAngle|" +
    "SolarIrradiance|SoundAlert|Stress|Survey|TargetPadIndex|TargetX|" +
    "TargetY|TargetZ|Temperature|TemperatureDifferentialEfficiency|" +
    "TemperatureExternal|TemperatureInput|TemperatureInput2|TemperatureOutput|" +
    "TemperatureOutput2|TemperatureSetting|Throttle|Thrust|ThrustToWeight|" +
    "Time|TimeToDestination|TotalMoles|TotalMolesInput|TotalMolesInput2|" +
    "TotalMolesOutput|TotalMolesOutput2|TotalQuantity|TrueAnomaly|" +
    "VelocityMagnitude|VelocityRelativeX|VelocityRelativeY|VelocityRelativeZ|" +
    "VelocityX|VelocityY|VelocityZ|Vertical|VerticalRatio|Volume|VolumeOfLiquid|" +
    "WattsReachingContact|Weight|WorkingGasEfficiency");

var logictypes = (
    "Acceleration|Activate|AirRelease|AlignmentError|Apex|AutoLand|" +
    "AutoShutOff|Average|Bpm|BurnTimeRemaining|Bypass|CelestialHash|" +
    "CelestialParentHash|Channel|Channel0|Channel1|Channel2|Channel3|" +
    "Channel4|Channel5|Channel6|Channel7|Charge|ChargeRatio|Class|" +
    "ClearMemory|CollectableGoods|Color|Combustion|CombustionInput|" +
    "CombustionInput2|CombustionLimiter|CombustionOutput|CombustionOutput2|" +
    "CompletionRatio|ContactTypeId|Contents|CurrentResearchPodType|Damage|" +
    "DestinationCode|DistanceAu|DistanceKm|DrillCondition|DryMass|" +
    "Eccentricity|Efficiency|ElevatorLevel|ElevatorSpeed|EntityState|" +
    "EnvironmentEfficiency|Error|ExhaustVelocity|ExportCount|" +
    "ExportQuantity|ExportSlotHash|ExportSlotOccupant|FilterType|" +
    "Filtration|FlightControlRule|Flush|ForceWrite|ForwardX|ForwardY|" +
    "ForwardZ|Fuel|Growth|Harvest|Health|Horizontal|HorizontalRatio|" +
    "Idle|ImportCount|ImportQuantity|ImportSlotHash|ImportSlotOccupant|" +
    "Inclination|Index|InterrogationProgress|LineNumber|Lock|" +
    "ManualResearchRequiredPod|Mass|Mature|MaxQuantity|Maximum|" +
    "MinWattsToContact|MineablesInQueue|MineablesInVicinity|Minimum|" +
    "MinimumWattsToContact|Mode|NextWeatherEventTime|None|OccupantHash|" +
    "Occupied|On|Open|OperationalTemperatureEfficiency|OrbitPeriod|" +
    "Orientation|Output|OverShootTarget|PassedMoles|Plant|" +
    "PlantEfficiency1|PlantEfficiency2|PlantEfficiency3|PlantEfficiency4|" +
    "PlantGrowth1|PlantGrowth2|PlantGrowth3|PlantGrowth4|PlantHash1|" +
    "PlantHash2|PlantHash3|PlantHash4|PlantHealth1|PlantHealth2|" +
    "PlantHealth3|PlantHealth4|PositionX|PositionY|PositionZ|Power|" +
    "PowerActual|PowerGeneration|PowerPotential|PowerRequired|" +
    "PrefabHash|Pressure|PressureAir|PressureEfficiency|PressureExternal|" +
    "PressureInput|PressureInput2|PressureInternal|PressureOutput|" +
    "PressureOutput2|PressureSetting|PressureWaste|Progress|Quantity|" +
    "Ratio|RatioCarbonDioxide|RatioCarbonDioxideInput|" +
    "RatioCarbonDioxideInput2|RatioCarbonDioxideOutput|" +
    "RatioCarbonDioxideOutput2|RatioLiquidCarbonDioxide|" +
    "RatioLiquidCarbonDioxideInput|RatioLiquidCarbonDioxideInput2|" +
    "RatioLiquidCarbonDioxideOutput|RatioLiquidCarbonDioxideOutput2|" +
    "RatioLiquidNitrogen|RatioLiquidNitrogenInput|" +
    "RatioLiquidNitrogenInput2|RatioLiquidNitrogenOutput|" +
    "RatioLiquidNitrogenOutput2|RatioLiquidNitrousOxide|" +
    "RatioLiquidNitrousOxideInput|RatioLiquidNitrousOxideInput2|" +
    "RatioLiquidNitrousOxideOutput|RatioLiquidNitrousOxideOutput2|" +
    "RatioLiquidOxygen|RatioLiquidOxygenInput|RatioLiquidOxygenInput2|" +
    "RatioLiquidOxygenOutput|RatioLiquidOxygenOutput2|" +
    "RatioLiquidPollutant|RatioLiquidPollutantInput|" +
    "RatioLiquidPollutantInput2|RatioLiquidPollutantOutput|" +
    "RatioLiquidPollutantOutput2|RatioLiquidVolatiles|" +
    "RatioLiquidVolatilesInput|RatioLiquidVolatilesInput2|" +
    "RatioLiquidVolatilesOutput|RatioLiquidVolatilesOutput2|" +
    "RatioNitrogen|RatioNitrogenInput|RatioNitrogenInput2|" +
    "RatioNitrogenOutput|RatioNitrogenOutput2|RatioNitrousOxide|" +
    "RatioNitrousOxideInput|RatioNitrousOxideInput2|" +
    "RatioNitrousOxideOutput|RatioNitrousOxideOutput2|RatioOxygen|" +
    "RatioOxygenInput|RatioOxygenInput2|RatioOxygenOutput|" +
    "RatioOxygenOutput2|RatioPollutant|RatioPollutantInput|" +
    "RatioPollutantInput2|RatioPollutantOutput|RatioPollutantOutput2|" +
    "RatioSteam|RatioSteamInput|RatioSteamInput2|RatioSteamOutput|" +
    "RatioSteamOutput2|RatioVolatiles|RatioVolatilesInput|" +
    "RatioVolatilesInput2|RatioVolatilesOutput|RatioVolatilesOutput2|" +
    "RatioWater|RatioWaterInput|RatioWaterInput2|RatioWaterOutput|" +
    "RatioWaterOutput2|ReEntryAltitude|Reagents|Recipe|RecipeHash|" +
    "ReferenceId|RequestHash|Required|RequiredPower|ReturnFuelCost|Rpm|" +
    "Seeding|SemiMajorAxis|Setting|SettingInput|SettingInputHash|" +
    "SettingOutput|SettingOutputHash|SignalID|SignalStrength|SizeX|SizeY|" +
    "SizeZ|SolarAngle|SolarConstant|SolarIrradiance|SortingClass|" +
    "SoundAlert|Stress|Sum|TargetPadIndex|TargetX|TargetY|TargetZ|" +
    "Temperature|TemperatureDifferentialEfficiency|TemperatureExternal|" +
    "TemperatureInput|TemperatureInput2|TemperatureOutput|" +
    "TemperatureOutput2|TemperatureSetting|Throttle|Thrust|ThrustToWeight|" +
    "Time|TimeToDestination|TotalMoles|TotalMolesInput|TotalMolesInput2|" +
    "TotalMolesOutput|TotalMolesOutput2|TrueAnomaly|Unknown|" +
    "VelocityMagnitude|VelocityRelativeX|VelocityRelativeY|" +
    "VelocityRelativeZ|VelocityX|VelocityY|VelocityZ|Vertical|" +
    "VerticalRatio|Volume|VolumeOfLiquid|WattsReachingContact|Weight" +
    "|WorkingGasEfficiency"
);

var logicslottypes = (
    "Charge|ChargeRatio|Class|Damage|Efficiency|FilterType|Growth|" +
    "Health|LineNumber|Lock|Mature|MaxQuantity|None|OccupantHash|" +
    "Occupied|On|Open|PrefabHash|Pressure|PressureAir|PressureWaste|" +
    "Quantity|ReferenceId|Seeding|SortingClass|Temperature|Volume"
);

var batchmodes = ("Average|Maximum|Minimum|Sum");

var reagentmodes = ("Contents|Recipe|Required|TotalContents");

var constants = ("nan|pinf|ninf|pi|deg2rad|rad2deg|epsilon");

var deprecated = (
    "ExportSlotHash|ExportSlotOccupant|ImportSlotHash|ImportSlotOccupant|" +
    "LogicType.ExportSlotHash|LogicType.ExportSlotOccupant|LogicType.ImportSlotHash|" +
    "LogicType.ImportSlotOccupant|LogicType.PlantEfficiency1|LogicType.PlantEfficiency2|" +
    "LogicType.PlantEfficiency3|LogicType.PlantEfficiency4|LogicType.PlantGrowth1|" +
    "LogicType.PlantGrowth2|LogicType.PlantGrowth3|LogicType.PlantGrowth4|" +
    "LogicType.PlantHash1|LogicType.PlantHash2|LogicType.PlantHash3|LogicType.PlantHash4|" +
    "LogicType.PlantHealth1|LogicType.PlantHealth2|LogicType.PlantHealth3|LogicType.PlantHealth4|" +
    "PlantEfficiency1|PlantEfficiency2|PlantEfficiency3|PlantEfficiency4|" +
    "PlantGrowth1|PlantGrowth2|PlantGrowth3|PlantGrowth4|PlantHash1|PlantHash2|" +
    "PlantHash3|PlantHash4|PlantHealth1|PlantHealth2|PlantHealth3|PlantHealth4"
);

// regexp must not have capturing parentheses. Use (?:) instead.
// regexps are ordered -> the first match is used
var rules = {
    start: [{
        token: "comment.line.number-sign",
        regex: /#.*$/,
    }, {
        token: ["text", "keyword", "text", "entity.name.tag"],
        regex: /^(\s*)(j(?:al)?)(\s+)([a-zA-Z_.][a-zA-Z0-9_.]*)/,
        comment: "absolute jumps | branches with a label"
    }, {
        token: "keyword",
        regex: "^\\s*(?:" + ops + ")\\b",
    }, {
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
        regex: "\\b(?:" + logictypes + "|" + logicslottypes + "|" + batchmodes + "|" + reagentmodes + ")\\b",
    }, {
        token: "invalid.deprecated",
        regex: "\\b(?:" + deprecated + ")\\b",
    }, 
    {
        token: "constant.language",
        regex: "\\b(?:" + constants + ")\\b",
    }, {
        token: "variable.language",
        regex: "\\b(?:" + enums + ")\\b",
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

export { rules };
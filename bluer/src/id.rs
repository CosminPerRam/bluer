//! Manufacturer ids and assigned UUIDs for service classes and profiles,
//! GATT services, GATT characteristics and GATT descriptors.
//!
//! The data herein is provided in part by the [Bluetooth numbers database]
//! created by Nordic Semiconductor ASA.
//!
//! [Bluetooth numbers database]: https://github.com/NordicSemiconductor/bluetooth-numbers-database

use std::convert::TryFrom;
use strum::{Display, EnumString};
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/service_class.inc"));

// =========================================================================================
//
// Copyright (c) 2019 - 2020, Nordic Semiconductor ASA
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this
//    list of conditions and the following disclaimer in the documentation and/or other
//    materials provided with the distribution.
//
// 3. Neither the name of Nordic Semiconductor ASA nor the names of its contributors may
//    be used to endorse or promote products derived from this software without specific
//    prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY, AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
// SHALL NORDIC SEMICONDUCTOR ASA OR CONTRIBUTORS BELIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
// TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
// ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

include!(concat!(env!("OUT_DIR"), "/service.inc"));
include!(concat!(env!("OUT_DIR"), "/characteristic.inc"));
include!(concat!(env!("OUT_DIR"), "/descriptor.inc"));
include!(concat!(env!("OUT_DIR"), "/company.inc"));

//
// =========================================================================================

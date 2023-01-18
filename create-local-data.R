
tmp <- tempfile()

download.file(
	"https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_timeseries_fips_37081_external",
#"https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_latest_external_data",
tmp,
quiet = TRUE,
cacheOK = FALSE
)

tmp2 <- tempfile()

download.file(
	"https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_timeseries_by_state_fips_37",
#"https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_latest_external_data",
tmp2,
quiet = TRUE,
cacheOK = FALSE
)

tmp3 <- tempfile()
download.file(
"https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_latest_by_state_fips_37",
tmp3,
quiet = TRUE,
cacheOK = FALSE
)

quick_json_read <- function(x){
  RcppSimdJson::fload(x)[[2]]
}

nc_vax <- quick_json_read(tmp)

head(nc_vax)

all_dat <- quick_json_read(tmp2)

all_state <- quick_json_read(tmp3)

head(all_state)
library(data.table)

setDT(all_dat)
setDT(nc_vax)


head(all_dat)

head(nc_vax)

all_dat[State_name == "North Carolina" & County == "Forsyth County"]


"https://data.cdc.gov/resource/7pvw-pdbr.json?state=North"

names(nc_vax)

tail(nc_vax, 20)


########################################################################################################################
#                                                                                                                      #
#   created by: MPZinke                                                                                                #
#   on 2022.05.17                                                                                                      #
#                                                                                                                      #
#   DESCRIPTION:                                                                                                       #
#   BUGS:                                                                                                              #
#   FUTURE:                                                                                                            #
#                                                                                                                      #
########################################################################################################################


all:
	# As per https://www.rust-lang.org/tools/install
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	sudo apt-get install postgresql libssl-dev -y
	# Ensure https://stackoverflow.com/questions/18664074/getting-error-peer-authentication-failed-for-user-postgres-when-trying-to-ge
	sudo systemctl start postgresql.service

	sudo -u postgres createuser root
	sudo -u postgres createdb -O root LightSetter
	sudo psql -d LightSetter -f ../DB/Schema.sql
	sudo psql -d LightSetter -f ../DB/Data.sql

	bash ExportEnvironmentVariables.sh
	cd ../Source
	cargo build

	sudo cp target/debug/LightSetter /usr/local/bin/
	sudo cp ../Install/LightSetter.service /etc/systemd/system/
	sudo systemctl enable LightSetter.service
	sudo systemctl start LightSetter.service

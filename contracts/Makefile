$MAKEFILES = $(shell find . -maxdepth 3 -type f -name Makefile)
# SUBDIRS   = $(filter-out ./,$(dir $($MAKEFILES)))
SUBDIRS = notes_nft signaturit_oracle escrow_test escrow
default: build

all: test

build:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir build || break; \
	done

test: build
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir test || break; \
	done

fmt:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir fmt || break; \
	done

clean:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir clean || break; \
	done

deploy: build
	$(eval ADMIN_ADDRESS := $(shell soroban keys address $(ACCOUNT)))
	@echo "-- Admin address: $(ADMIN_ADDRESS)"

	@# Deploy Escrow contract
	$(eval ESCROW_ADDRESS := $(shell soroban contract deploy \
	--wasm ../target/wasm32-unknown-unknown/release/escrow.optimized.wasm \
	--source $(ACCOUNT) \
	--network testnet \
	))
	@echo "-- Escrow address: $(ESCROW_ADDRESS)"

	@# Deploy Signaturit Oracle contract
	$(eval ORACLE_ADDRESS := $(shell soroban contract deploy \
	--wasm ../target/wasm32-unknown-unknown/release/signaturit_oracle.optimized.wasm \
	--source $(ACCOUNT) \
	--network testnet \
	))
	@echo "-- Signaturit Oracle address: $(ORACLE_ADDRESS)"

	@# Deploy NFT Notes contract
	$(eval NFT_ADDRESS := $(shell soroban contract deploy \
	--wasm ../target/wasm32-unknown-unknown/release/notes_nft.optimized.wasm \
	--source $(ACCOUNT) \
	--network testnet \
	))
	@echo "-- NFT Notes address: $(NFT_ADDRESS)"

	@echo "\nSend initializations..."

	@# Escrow Contract Initialization
	@soroban contract invoke --id $(ESCROW_ADDRESS) --source $(ACCOUNT) --network testnet -- initialize --asset_address $(ASSET) --oracle_address $(ORACLE_ADDRESS) --nft_notes_address $(NFT_ADDRESS)
	@echo -n "✔️ " && echo "Escrow initialized"

	@# Signaturit Oracle Contract Initialization
	@soroban contract invoke --id $(ORACLE_ADDRESS) --source $(ACCOUNT) --network testnet -- initialize --admin $(ADMIN_ADDRESS)
	@echo -n "✔️ " && echo "Signaturit Oracle initialized"

	@# NFT Notes Contract Initialization
	@soroban contract invoke --id $(NFT_ADDRESS) --source $(ACCOUNT) --network testnet -- initialize --admin $(ESCROW_ADDRESS) --name "$(NFT_NAME)" --symbol "$(NFT_SYMBOL)"
	@echo -n "✔️ " && echo "NFT Notes initialized\n"

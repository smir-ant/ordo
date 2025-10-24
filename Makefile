I18N_DIR = i18n
POT_FILE = $(I18N_DIR)/messages.pot
LANGS = en ru
SLINT_FILES = $(shell find ./ -name "*.slint")

default: help

help:
	@echo "  i18n"
	@echo "    i18n_init      - Create i18n folders with init state"
	@echo "    i18n_sync      - Update .pot and .po"
	@echo "    i18n_unfuzzy   - Remove fuzzy flugs"
	@echo ""
	@echo "  To work with localization, gettext utilities must be installed. (msgmerge, msginit etc)."
	@echo "  На macOS: brew install gettext"


_i18n_update_pot:
	@slint-tr-extractor -o $(POT_FILE) $(SLINT_FILES)

i18n_init:
	@mkdir -p $(I18N_DIR)
	$(MAKE) _i18n_update_pot
	@$(foreach lang, $(LANGS), \
		mkdir -p $(I18N_DIR)/$(lang)/LC_MESSAGES; \
		msginit --no-translator --input=$(POT_FILE) --locale=$(lang) --output-file=$(I18N_DIR)/$(lang)/LC_MESSAGES/ordo.po; \
	)

i18n_sync:
	$(MAKE) _i18n_update_pot
	# update .po (from .pot) without fuzzy matching and then remove #~ (obsolete entries)
	@$(foreach lang, $(LANGS), \
		msgmerge --update --backup=none --no-fuzzy-matching $(I18N_DIR)/$(lang)/LC_MESSAGES/ordo.po $(POT_FILE); \
		msgattrib --no-obsolete --output-file=$(I18N_DIR)/$(lang)/LC_MESSAGES/ordo.po $(I18N_DIR)/$(lang)/LC_MESSAGES/ordo.po; \
	)
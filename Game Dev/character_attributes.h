class CharacterAttributes {
public:
    struct BaseStats {
        float speed;
        float strength;
        float shooting;
        float ballControl;
        float defense;
        float stamina;
    };

    struct SpecialAbilities {
        // Signature moves and abilities
        vector<string> signatureMoves;
        vector<string> specialAnimations;
        vector<string> uniqueCelebrations;
        float gamebreakeMeter;
        float teamChemistry;
    };

    struct CharacterCustomization {
        // Visual customization
        string modelPath;
        vector<string> accessories;
        vector<string> animations;
        vector<string> effects;
        bool isLegendary;
    };
}; 
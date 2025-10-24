use rand::rng;
use rand::seq::SliceRandom;

#[allow(dead_code)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub name: String,
    pub skill: u32,
    #[serde(default)]
    pub is_captain: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::new(),
            skill: 0,
            is_captain: false,
        }
    }
}

pub fn sum_skill(team: &Vec<Player>) -> u32 {
    team.iter().map(|p| p.skill).sum::<u32>() / 2
}

fn combinations(n: usize) -> Vec<Vec<usize>> {
    let k = n / 2;
    let mut result = Vec::new();
    let mut combo = Vec::with_capacity(k);
    fn backtrack(
        start: usize,
        n: usize,
        k: usize,
        combo: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if combo.len() == k {
            result.push(combo.clone());
            return;
        }
        for i in start..n {
            combo.push(i);
            backtrack(i + 1, n, k, combo, result);
            combo.pop();
        }
    }
    backtrack(0, n, k, &mut combo, &mut result);
    result
}

pub fn best_balanced_split(
    players: &mut [Player],
    captain1_idx: Option<usize>,
    captain2_idx: Option<usize>,
) -> (Vec<Player>, Vec<Player>, i32) {
    let mut rng = rng();
    // players.shuffle(&mut rng);

    let total_players = players.len();
    assert_eq!(
        total_players % 2,
        0,
        "Número de jogadores deve ser par para dividir em times iguais"
    );

    // Se houver capitães, processar de forma especial
    if let (Some(cap1), Some(cap2)) = (captain1_idx, captain2_idx) {
        // Garantir que os capitães sejam diferentes
        if cap1 == cap2 {
            panic!("Os capitães devem ser jogadores diferentes");
        }

        let captain1 = players[cap1].clone();
        let captain2 = players[cap2].clone();

        // Criar lista de jogadores sem os capitães
        let remaining_players: Vec<(usize, Player)> = players
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != cap1 && *idx != cap2)
            .map(|(idx, p)| (idx, p.clone()))
            .collect();

        let remaining_count = remaining_players.len();
        let all_combinations = combinations(remaining_count);

        let mut smallest_skill_difference = i32::MAX;
        let mut most_balanced_team1 = Vec::new();
        let mut most_balanced_team2 = Vec::new();

        for combination in all_combinations {
            let mut is_in_team1 = vec![false; remaining_count];
            for &player_index in &combination {
                is_in_team1[player_index] = true;
            }

            let (team1_remaining, team2_remaining): (Vec<_>, Vec<_>) = remaining_players
                .iter()
                .enumerate()
                .partition(|(index, _)| is_in_team1[*index]);

            let mut team1: Vec<Player> = team1_remaining
                .into_iter()
                .map(|(_, (_, player))| player.clone())
                .collect();
            let mut team2: Vec<Player> = team2_remaining
                .into_iter()
                .map(|(_, (_, player))| player.clone())
                .collect();

            // Adicionar capitães aos times
            team1.insert(0, captain1.clone());
            team2.insert(0, captain2.clone());

            let team1_skill = sum_skill(&team1) as i32;
            let team2_skill = sum_skill(&team2) as i32;
            let skill_difference = (team1_skill - team2_skill).abs();

            if skill_difference < smallest_skill_difference {
                smallest_skill_difference = skill_difference;
                most_balanced_team1 = team1;
                most_balanced_team2 = team2;
            }
        }

        return (
            most_balanced_team1,
            most_balanced_team2,
            smallest_skill_difference,
        );
    }

    // Código original quando não há capitães
    let all_combinations = combinations(total_players);

    let mut smallest_skill_difference = i32::MAX;
    let mut most_balanced_team1 = Vec::new();
    let mut most_balanced_team2 = Vec::new();

    for combination in all_combinations {
        let mut is_in_team1 = vec![false; total_players];
        for &player_index in &combination {
            is_in_team1[player_index] = true;
        }

        let (team1, team2): (Vec<_>, Vec<_>) = players
            .iter()
            .enumerate()
            .partition(|(index, _)| is_in_team1[*index]);

        let team1: Vec<Player> = team1
            .into_iter()
            .map(|(_, player)| player.clone())
            .collect();
        let team2: Vec<Player> = team2
            .into_iter()
            .map(|(_, player)| player.clone())
            .collect();

        let team1_skill = sum_skill(&team1) as i32;
        let team2_skill = sum_skill(&team2) as i32;
        let skill_difference = (team1_skill - team2_skill).abs();

        if skill_difference < smallest_skill_difference {
            smallest_skill_difference = skill_difference;
            most_balanced_team1 = team1;
            most_balanced_team2 = team2;
        }
    }

    (
        most_balanced_team1,
        most_balanced_team2,
        smallest_skill_difference,
    )
}

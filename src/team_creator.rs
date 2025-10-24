use rand::rng;
use rand::seq::SliceRandom;
use rand::Rng;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
    team.iter().map(|p| p.skill).sum::<u32>()
}

pub fn best_balanced_split(
    players: &mut [Player],
    team_count: usize,
) -> Vec<Vec<Player>> {
    let mut rng = rng();
    players.shuffle(&mut rng);

    // Separar capitães e jogadores normais
    let (captains, non_captains): (Vec<Player>, Vec<Player>) = players
        .iter()
        .cloned()
        .partition(|p| p.is_captain);

    // Determinar qual lista usar para combinações
    let (base_players, has_captains) = if captains.len() == team_count {
        (non_captains.clone(), true)
    } else {
        (players.to_vec(), false)
    };

    if team_count == 1 {
        return vec![base_players];
    }

    // Para divisão em múltiplos times, usamos uma abordagem gulosa iterativa
    // que é mais eficiente que enumerar todas as combinações possíveis
    let mut best_teams = create_greedy_teams(&base_players, team_count, &captains, has_captains);
    let mut best_variance = calculate_variance(&best_teams);

    // Tenta algumas iterações de otimização para melhorar o balanceamento
    let iterations = if base_players.len() <= 10 { 1000 } else { 100 };

    for _ in 0..iterations {
        let mut shuffled = base_players.clone();
        shuffled.shuffle(&mut rng);
        let teams = create_greedy_teams(&shuffled, team_count, &captains, has_captains);
        let variance = calculate_variance(&teams);

        if variance < best_variance {
            best_variance = variance;
            best_teams = teams;
        }
    }

    best_teams
}

fn create_greedy_teams(
    base_players: &[Player],
    team_count: usize,
    captains: &[Player],
    has_captains: bool,
) -> Vec<Vec<Player>> {
    let mut rng = rng();

    let mut teams: Vec<Vec<Player>> = if has_captains {
        captains.iter().map(|cap| vec![cap.clone()]).collect()
    } else {
        vec![Vec::new(); team_count]
    };

    // Calculate target team size
    let target_size = if has_captains {
        (base_players.len() / team_count) + 1  // +1 for the captain already in each team
    } else {
        base_players.len() / team_count
    };

    // Shuffle players for randomness instead of strict sorting
    let mut shuffled_players = base_players.to_vec();
    shuffled_players.shuffle(&mut rng);

    // Distribuir jogadores com randomização
    for player in shuffled_players {
        // Get available teams (not full)
        let available_teams: Vec<usize> = teams
            .iter()
            .enumerate()
            .filter(|(_, team)| team.len() < target_size)
            .map(|(idx, _)| idx)
            .collect();

        if available_teams.is_empty() {
            break;
        }

        // Add some randomness: 50% of the time pick team with lowest skill,
        // 50% of the time pick randomly from available teams
        let team_idx = if rng.random::<f32>() < 0.5 && available_teams.len() > 1 {
            // Pick team with lowest skill among available
            *available_teams
                .iter()
                .min_by_key(|&&idx| sum_skill(&teams[idx]))
                .unwrap()
        } else {
            // Pick random available team
            let random_idx = rng.random_range(0..available_teams.len());
            available_teams[random_idx]
        };

        teams[team_idx].push(player);
    }

    teams
}

fn calculate_variance(teams: &[Vec<Player>]) -> i32 {
    let skills: Vec<i32> = teams.iter().map(|team| sum_skill(team) as i32).collect();

    if teams.len() == 2 {
        (skills[0] - skills[1]).abs()
    } else {
        let avg = skills.iter().sum::<i32>() / skills.len() as i32;
        skills.iter().map(|&s| (s - avg).pow(2)).sum()
    }
}

use crate::cache::PersistentCache;
use eframe::egui;

mod audio_player;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "template_cache.ron"
    }
}

pub struct Lambidinha {
    pd: PersistentData,
    audios: Vec<audio_player::Player>,
}

impl Default for Lambidinha {
    fn default() -> Self {
        Lambidinha {
            pd: PersistentData::read_or(PersistentData {}),
            audios: vec![],
        }
    }
}

impl super::View for Lambidinha {
    fn name(&self) -> &str {
        "💥 Lambidinha"
    }

    fn ui(&mut self, ui: &mut egui::Ui, _settings: &mut crate::app::SettingsData) {
        self.load_audios();
        
        ui.heading("Lambidinha chorando para o Miojo1337 no mix entre amigos");
        ui.separator();

        ui.vertical_centered(|ui| {
            ui.set_max_width(600.0);
            ui.vertical(|ui| {
                self.audios.iter_mut().enumerate().for_each(|(i, player)| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Áudio {}: ", i + 1));
                        player.ui(ui);
                    });
                    ui.add_space(10.0);
                });
                ui.heading("[1º áudio]");
                ui.label("É galera, eu vou mandar real pra vocês, assim ó: eu garanto pra todos vocês que sim, sem a menor dúvida, o Miojo xita e na cara dura, contra todos vocês, contra mim e contra todo mundo que tá aí nessa merda desse mix, entendeu? Cara de pau do caralho, o cara xitar e dizer que não tá xitando contra amigos, entendeu? Isso aí, cara, eu nunca vi tanta cara de pau na minha vida. E assim ó, eu já perdi as contas de quantos retakes eu tava ali no servidor, às vezes eu nem tava no Discord, eu tava ali só telando o Miojo, cara, assim como hoje eu fiz ali ó, só que eu tava no Discord.");
                ui.add_space(10.0);

                ui.label("Muito difícil ter um round onde ele estivesse normal, onde o que ele fizesse era normal, nunca. As balas dele, a mira dele é normal? Nunca, não dá pra dizer. Mas assim ó, a maioria das vezes é claramente tá xitando, claramente tá de wall, aí agora essa merda que ele fez ali no meio não tem nenhuma lógica, nenhuma explicação, nada. É puro wall hack, puro e simples wall hack. É isso que esse filho da puta aí tá fazendo.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[2º áudio]");
                ui.label("Acredito que a maioria de vocês aqui duvida que ele tá realmente fazendo isso, mas eu tenho certeza que a maioria que tá aqui nesse grupo já suspeitou e suspeitou muito fortemente dele. Algumas jogadas muito específicas que era tipo assim, era a coisa mais bizarra da vida de se ver. Algumas não tão bizarras mas, o cara praticamente dizendo na tela dele que o cara tá de wallhack.");
                ui.add_space(10.0);

                ui.label("E não foi diferente hoje no retake, no servidor de retake que a gente tava jogando. Galera jogando retake ali é de boa, e ele xitando. Alguns rounds o que ele fazia era normal, mas vários rounds o que ele fazia não tinha lógica nenhuma, de se quer alguém na vida pensar em fazer o que ele fez. Enfim, eu gravei e gravei pela Steam, depois eu vou ver se vai aparecer na Steam pra mostrar pra todo mundo que tá aqui. Porque assim ó. E não foi só hoje. Teve vezes que eu apareci ali, eu fiquei olhando a tela da galera aí, especificamente a tela dele, e só analisando, só analisando, vendo bizarrice atrás e bizarrice.");
                ui.add_space(10.0);

                ui.label("Hoje eu tava no retake, nem joguei, só fiquei telando ele e olhando e confirmando todas as minhas suspeitas de que sim, de fato o Miojo é wall contra a gente no mix no servidor privado. Porque quando a gente cai em GC, quando a gente cai em Faceit, aí ele não tá xitando, ou se xita, talvez disfarça muito bem. Mas eu acho que de fato ele não xita, talvez ele não tenha um programa bom o suficiente pra poder xitar lá sem tomar ban. Então sim, lá ele não xita, aí ele joga bem, e aí só que nada nele é suspeito, tudo é normal, tudo é sossegado, tranquilo quando a gente tá em GC, quando a gente tá em Faceit. Agora, servidor privado aqui ó, é só coisa bizarra na tela dele. Essa é a parada.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[3º áudio]");
                ui.label("Então eu espero que vocês caiam na real com esse palhaço do Miojo. Porque se ele joga bem, talvez ele jogue bem na Faceit e na GC. Agora, quando cai no mix no servidor privado pra jogar com os amigos, aí ele não quer jogar limpo.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[4º áudio]");
                ui.label("Sim, exatamente, eu estou falando sério, porque o negócio é o seguinte: servidor privado, qualquer um aqui pode xitar à vontade que não vai tomar ban, porque é servidor privado, e ele está se aproveitando disso. Qualquer um aqui denunciar ele na Steam, em qualquer coisa, não vai nem aparecer lá na central da Steam a denúncia, porque é servidor privado. Então assim, não tem risco nenhum xitar em servidor privado, e ele está se aproveitando disso.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[5º áudio]");
                ui.label("Não é à toa que na semana passada terminou o mix e teve algumas pessoas que foram ali numa salinha do Discord mostrar as gravações em vídeo feitos da tela dele e pedir pra ele explicar qual que era a lógica dele fazer o que ele fez ali, porque estavam suspeitando daquele comportamento dele, estavam suspeitando daquela jogada dele.");
                ui.add_space(10.0);

                ui.label("A maioria das pessoas que estão aqui no grupo já suspeitaram do Miojo, da jogabilidade dele contra a gente no mix, no servidor privado. A maioria das pessoas que estão aqui tem comentado por fora, alguns tem falado com ele porque suspeitam dele jogando contra a gente no mix, no servidor privado. A maioria que tá aqui. Só que o detalhe é que a maioria dessas pessoas que suspeitam dele e acham muito bizarro quando olham pra tela dele, muitas vezes, estão certas. Porque sim, ele está xitando. Ele está se aproveitando do fato de estar no servidor privado, que não toma ban de jeito nenhum, e aí tá cheatando contra a gente. Só que quando cai GC, cai Faceit, ele não xita, e aí, de fato, ele joga bem, porque ele joga bem, só que ele não faz o que ele faz aqui no mix.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[6º áudio]");
                ui.label("Já faz um bom tempo que eu tô conversando com algumas pessoas aqui falando, ó, eu só acredito de fato que ele joga desse jeito, fazendo essas bizarrices que ele faz muitas vezes, o dia que eu jogar contra ele em LAN, frente a frente, vendo que ele não tem nada instalado, vendo que ele tá jogando com a tela limpa, porque aqui eu tenho certeza que ele usa o wallhack contra a gente num mix no servidor privado.");
                ui.add_space(10.0);

                ui.label("Eu mandei uns áudios antes aqui, mas demorou pra chegar, uns nem chegaram, porque meu celular é uma merda. Mas assim ó, eu jogo, eu já joguei com o Miojo há uns, sei lá, dois anos atrás, três anos atrás, tava o Night, tava o Storm, e tinha mais gente junto. Desde aquela época, tanto eu quanto outras pessoas suspeitavam dele às vezes, é claro que não sempre, igual hoje. Ó, hoje o pessoal ali tava, tinha quatro jogando retake, tava eu e mais um ali fora, só assistindo. Eu só tava telando Miojo e gravando, inclusive, porque o suspeito dele já faz tempo.");
                ui.add_space(10.0);

                ui.label("Diferente das partidas que eu pego por aí, que qualquer coisinha estranha eu já chamo de xitado, não, eu tô analisando esse palhaço faz umas três semanas já. Desde que eu comecei a jogar mix aí com a galera, eu tô suspeitando dele, tô analisando ele, tô vendo o retake o tempo inteiro, toda vez que eu tenho a oportunidade de estar nesse servidor aí, quando tá rolando o retakezinho, tô na tela dele. Se eu tô jogando competitivo e eu tô no time dele, eu tô na tela dele. Então toda hora vendo coisa bizarra.");
                ui.add_space(10.0);

                ui.label("E se tem gente aqui que realmente defende cada bizarrice que ele faz sem questionar, pô, tá na hora de amadurecer um pouco, porque isso daí é coisa de criança, no mínimo. No mínimo, todo mundo aqui tem que ter um mínimo de senso crítico pra pelo menos pensar antes de acreditar em qualquer coisa que alguém fala.");
                ui.add_space(10.0);

                ui.label("Semana passada, depois do mix, teve um dia ali que teve um mix que, depois do mix, teve um pessoal que se juntou numa salinha junto com ele ali no Discord, pra pegar as gravação das bizarrice que ele fez no mix e perguntar pra ele: \"Ó, veja isso daqui, me explica isso daqui\", porque estavam suspeitando dele no mix no servidor privado contra os amigos.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[7º áudio]");
                ui.label("O negócio é que hoje eu estava desde as 7h30 lá no servidor de retake que vocês estavam jogando. Eu estava lá, ao invés de entrar para jogar, eu preferi ficar de fora só telando ele, para mais uma vez estudar o que ele faz, analisar o que ele faz, olhar na tela dele. Porque se a tela dele estivesse normal, porque muitas vezes acontece isso. De fato, muitas vezes eu tomo uma bala muito estranha de um cara lá na GC ou na Faceit, eu acho que é cheater, depois eu vou lá gastar meu tempo na minha vida para ficar olhando o demo, eu vou na tela do cara e eu vejo, pô, realmente não dá para dizer que o cara está xitado; quando eu estou na tela dele, é muito difícil eu confirmar que o cara está xitando quando eu vou na tela do cara. Às vezes contra é estranho, mas quando eu vou na tela do cara é normal, de fato. Só que na tela do Miojo é sempre bizarro, bizarro. As coisas que ele faz é bizarro.");
                ui.add_space(10.0);

                ui.label("Novamente, talvez ele não xita em GC e em Faceit, porque realmente é difícil de xitar e lá perde a conta. No servidor privado do MM, tu pode fazer o que você quiser, que não foge de nenhuma regra, que você não vai perder a conta. Todo mundo, cara, pode pegar todo mundo aqui e jogar HvH nesse servidor aí, todo mundo denunciar, todo mundo, ninguém vai cair porque é servidor privado. Não vai nem aparecer denúncia lá na Steam porque é servidor privado.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.heading("[8º áudio]");
                ui.label("O negócio é que, se vocês têm essa paciência, ótimo, perfeito, se divirtam. Porque vocês têm realmente essa paciência de estar ali vendo uma bizarrice na tela dele, às vezes, ou tomando uma bala muito estranha dele, e relevar, achar que o cara realmente é um prodígio. Quer dizer, digo, achar não, criar essa ilusão na cabeça de achar que o cara é um prodígio, que o cara joga bem demais e que o cara deveria estar no Major por ter tanta habilidade, por ter tanta noção de jogo. Se vocês têm essa paciência, perfeito, se divirtam.");
                ui.add_space(10.0);

                ui.label("Só que eu, infelizmente, eu queria ter essa paciência para não criar essa bagunça que está acontecendo aqui agora. Eu queria ter essa paciência para não criar essa confusãozinha aqui e simplesmente ignorar, relevar e continuar jogando no modo foda-se: \"Vai, ah, tá bom, ele está xitado, foda-se, não estou nem aí, vamos continuar jogando, é tudo amigo\". Tudo amigo o caralho, porque amigo não faz essa merda aí, velho.");
                ui.add_space(10.0);

                ui.label("Eu, particularmente, tenho certeza, certeza, 100%. Eu entrei, a gente entrou no comp agora, eu já tinha certeza que ele estava xitado depois que eu olhei o retake hoje, que eu, como eu falei, eu já estou acompanhando faz duas semanas já. Então, se vocês têm essa paciência, perfeito, se divirtam, eu estou vazando. Pode crer?");
                ui.add_space(10.0);

                ui.label("Mas assim, uma hora ou outra, a verdade, assim como eu tenho certeza que, para mim, está claro, para vocês vai se tornar claro e não vai demorar muito, que sim, de fato, definitivamente, esse cara xita no mix contra os amigos. Bem como eu já disse anteriormente nos áudios anteriores. Talvez ele não xite em GC e Faceit, porque não dá, talvez ele não tenha equipamento para isso. Mas aqui no mix, que pode xitar com qualquer programinha de merda aí que não dá ban, eu tenho certeza que ele xita.");
                ui.add_space(10.0);

                ui.label("Porque, de fato, tem uma diferença bem grande entre como que ele joga GC, Faceit e como que ele joga aqui no servidor privado. Só que assim ó, é no servidor privado, só que é contra os amigos. Amigo não fica com essa palhaçada, entendeu? Isso daí, na minha opinião, é um desvio de caráter com a maior cara de pau que eu já vi na minha vida. Eu espero que um dia a ficha caia para vocês. Se divirtam.");
            });
        });
    }
}

impl Lambidinha {
    fn load_audios(&mut self) {
        if self.audios.is_empty() {
            self.audios = vec![
                audio_player::Player::new(include_bytes!("../../../assets/audio 1.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 2.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 3.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 4.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 5.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 6.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 7.ogg").to_vec()),
                // audio_player::Player::new(include_bytes!("../../../assets/audio 8.ogg").to_vec()),
            ]
        }
    }
}

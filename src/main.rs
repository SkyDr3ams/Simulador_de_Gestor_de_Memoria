mod models;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use models::{AlgoritmoReemplazo, GestorMemoria, Proceso};
use rand::Rng;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    fs, io,
    time::{Duration, Instant},
};

const MAX_PROCESOS: usize = 30;

/// Configuración del sistema leída desde config.ini
struct Configuracion {
    tamaño_ram: usize,
    tamaño_swap: usize,
    tamaño_pagina: usize,
}

impl Configuracion {
    fn cargar(ruta: &str) -> Result<Self, String> {
        let contenido =
            fs::read_to_string(ruta).map_err(|_| "ERROR: Archivo config.ini no encontrado")?;

        let mut ram = 0;
        let mut swap = 0;
        let mut pagina = 0;

        for linea in contenido.lines() {
            if let Some((clave, valor)) = linea.split_once('=') {
                let v = valor.trim().parse::<usize>().unwrap_or(0);
                match clave.trim() {
                    "RAM_SIZE" => ram = v,
                    "SWAP_SIZE" => swap = v,
                    "PAGE_SIZE" => pagina = v,
                    _ => {}
                }
            }
        }

        if ram == 0 || pagina == 0 {
            return Err("ERROR: Configuración inválida (valores en cero)".to_string());
        }
        Ok(Configuracion {
            tamaño_ram: ram,
            tamaño_swap: swap,
            tamaño_pagina: pagina,
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    // 1. Cargar configuración
    let config = Configuracion::cargar("config.ini").expect("ERROR: Fallo al cargar config.ini");

    // 2. Mostrar menú de selección
    println!("====================================");
    println!("  SIMULADOR GESTOR DE MEMORIA v1.0");
    println!("====================================");
    println!();
    println!("Seleccione Algoritmo de Reemplazo:");
    println!("  1) FIFO (Primero en Entrar, Primero en Salir)");
    println!("  2) Reloj (Segunda Oportunidad)");
    println!("  3) LRU (Menos Recientemente Usado)");
    println!();
    print!("Ingrese su elección (1-3): ");
    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let algoritmo = match input.trim() {
        "1" => AlgoritmoReemplazo::FIFO,
        "2" => AlgoritmoReemplazo::Reloj,
        "3" => AlgoritmoReemplazo::LRU,
        _ => {
            println!("Elección inválida, usando FIFO por defecto");
            AlgoritmoReemplazo::FIFO
        }
    };

    println!();
    println!("Seleccione Modo de Simulación:");
    println!("  1) Automático (procesos creados/terminados automáticamente)");
    println!("  2) Manual (usted controla creación con tecla 'N')");
    println!();
    print!("Ingrese su elección (1-2): ");
    io::Write::flush(&mut io::stdout())?;

    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input)?;
    let modo_auto = match mode_input.trim() {
        "1" => true,
        "2" => false,
        _ => {
            println!("Elección inválida, usando Automático por defecto");
            true
        }
    };

    // 3. Preparar terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 4. Iniciar gestor de memoria
    let mut gestor = GestorMemoria::new(
        config.tamaño_ram,
        config.tamaño_swap,
        config.tamaño_pagina,
        algoritmo,
    );

    // 5. Ejecutar simulador
    let resultado = ejecutar_app(&mut terminal, &mut gestor, modo_auto);

    // 6. Limpiar terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = resultado {
        println!("Error: {:?}", err);
    }
    Ok(())
}

fn ejecutar_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    gestor: &mut GestorMemoria,
    mut modo_auto: bool,
) -> io::Result<()> {
    let velocidad_tick = Duration::from_millis(100);
    let mut ultimo_tick = Instant::now();
    let mut temporizador_accion = Instant::now();
    let mut contador_pid = 1;
    let mut pausado = false;

    loop {
        // A. Dibujar interfaz
        terminal.draw(|f| ui::dibujar(f, gestor, pausado, modo_auto))?;

        // B. Escuchar teclado
        let timeout = velocidad_tick
            .checked_sub(ultimo_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(tecla) = event::read()? {
                match tecla.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => {
                        pausado = !pausado;
                        let estado = if pausado { "PAUSADO" } else { "EJECUTANDO" };
                        gestor.registrar_log(&format!("[ESTADO] Sistema {}", estado));
                    }
                    KeyCode::Char('a') => {
                        modo_auto = !modo_auto;
                        let modo = if modo_auto { "AUTOMATICO" } else { "MANUAL" };
                        gestor.registrar_log(&format!("[MODO] Cambiado a modo {}", modo));
                    }
                    KeyCode::Char('n') => {
                        // Crear proceso manualmente (verificar límite)
                        if gestor.procesos.len() >= MAX_PROCESOS {
                            gestor
                                .registrar_log("[ERROR] Límite máximo de procesos (30) alcanzado");
                        } else {
                            crear_proceso(gestor, &mut contador_pid);
                        }
                    }
                    KeyCode::Char('k') => {
                        gestor.matar_proceso_aleatorio();
                    }
                    KeyCode::Char('1') => {
                        gestor.algoritmo = AlgoritmoReemplazo::FIFO;
                        gestor.registrar_log("[CONFIG] Algoritmo cambiado a FIFO");
                    }
                    KeyCode::Char('2') => {
                        gestor.algoritmo = AlgoritmoReemplazo::Reloj;
                        gestor.registrar_log("[CONFIG] Algoritmo cambiado a Reloj");
                    }
                    KeyCode::Char('3') => {
                        gestor.algoritmo = AlgoritmoReemplazo::LRU;
                        gestor.registrar_log("[CONFIG] Algoritmo cambiado a LRU");
                    }
                    _ => {}
                }
            }
        }

        // C. Simulación automática (solo si modo_auto Y no pausado)
        if modo_auto && !pausado && temporizador_accion.elapsed() >= Duration::from_millis(500) {
            let mut rng = rand::thread_rng();
            let decision = rng.gen_range(0..100);

            if decision < 60 && gestor.procesos.len() < MAX_PROCESOS {
                // 60%: Crear proceso (si no estamos en el límite)
                crear_proceso(gestor, &mut contador_pid);
            } else if decision < 90 {
                // 30%: Matar proceso
                gestor.matar_proceso_aleatorio();
            }
            // 10%: Idle

            temporizador_accion = Instant::now();
        }

        if ultimo_tick.elapsed() >= velocidad_tick {
            ultimo_tick = Instant::now();
        }
    }
}

/// Helper para crear un proceso con parámetros aleatorios
fn crear_proceso(gestor: &mut GestorMemoria, contador_pid: &mut usize) {
    let mut rng = rand::thread_rng();
    let tamaño = rng.gen_range(gestor.tamaño_pagina_kb..gestor.tamaño_pagina_kb * 5);
    let (r, g, b) = (
        rng.gen_range(50..255),
        rng.gen_range(50..255),
        rng.gen_range(50..255),
    );

    gestor.asignar_proceso(Proceso {
        pid: *contador_pid,
        nombre: format!("P_{}", contador_pid),
        tamaño_kb: tamaño,
        paginas: vec![],
        color: (r, g, b),
    });
    *contador_pid += 1;
}

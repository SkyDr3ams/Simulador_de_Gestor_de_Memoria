use crate::models::GestorMemoria;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renderizado principal de la interfaz estilo htop profesional
pub fn dibujar(f: &mut Frame, gestor: &GestorMemoria, pausado: bool, modo_auto: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top bar
            Constraint::Length(3), // Stats bar
            Constraint::Min(8),    // Main content
            Constraint::Length(8), // Bottom: Logs + Info
            Constraint::Length(1), // Footer bar
        ])
        .split(f.size());

    // TOP BAR
    dibujar_top_bar(f, gestor, pausado, modo_auto, chunks[0]);

    // STATS BAR
    dibujar_stats_bar(f, gestor, chunks[1]);

    // MAIN CONTENT
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[2]);

    dibujar_mapa_ram(f, gestor, main_layout[0]);
    dibujar_panel_info(f, gestor, main_layout[1]);

    // BOTTOM
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(chunks[3]);

    dibujar_logs(f, gestor, bottom[0]);
    dibujar_swap(f, gestor, bottom[1]);

    // FOOTER
    dibujar_footer(f, chunks[4]);
}

/// Barra superior con título y estado
fn dibujar_top_bar(
    f: &mut Frame,
    gestor: &GestorMemoria,
    pausado: bool,
    modo_auto: bool,
    area: ratatui::layout::Rect,
) {
    let estado = if pausado { "PAUSADO" } else { "EJECUTANDO" };
    let modo = if modo_auto { "AUTOMATICO" } else { "MANUAL" };

    let texto = format!(
        " Gestor de Memoria v1.0 | Algoritmo: {:?} | Modo: {} | Estado: {} ",
        gestor.algoritmo, modo, estado
    );

    let style = if pausado {
        Style::default().bg(Color::Yellow).fg(Color::Black).bold()
    } else {
        Style::default().bg(Color::Blue).fg(Color::White).bold()
    };

    let bar = Paragraph::new(texto)
        .style(style)
        .alignment(Alignment::Left);

    f.render_widget(bar, area);
}

/// Barra de estadísticas principales
fn dibujar_stats_bar(f: &mut Frame, gestor: &GestorMemoria, area: ratatui::layout::Rect) {
    let total_marcos = gestor.marcos_ram.len();
    let marcos_usados = total_marcos - gestor.contar_marcos_libres();
    let utilizacion = gestor.utilización_ram();
    let tasa_fallos = gestor.tasa_fallos();

    let stats = vec![
        format!("Processes: {}/{}", gestor.procesos.len(), 30),
        format!(
            "RAM Usage: {}/{} frames ({:.1}%)",
            marcos_usados, total_marcos, utilizacion
        ),
        format!(
            "Page Faults: {} ({:.2}%)",
            gestor.fallos_pagina, tasa_fallos
        ),
        format!("Swaps: {}", gestor.swaps_realizados),
    ];

    let bloque = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(" System Statistics ")
        .style(Style::default().fg(Color::Cyan));

    let parrafo = Paragraph::new(stats.join(" | "))
        .block(bloque)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    f.render_widget(parrafo, area);
}

/// Mapa de memoria RAM estilo tabla
fn dibujar_mapa_ram(f: &mut Frame, gestor: &GestorMemoria, area: ratatui::layout::Rect) {
    let bloque = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" Mapa de Memoria Física (RAM) ")
        .style(Style::default().fg(Color::Green));

    let area_interna = bloque.inner(area);
    f.render_widget(bloque, area);

    // Grid de 8 columnas para mejor aprovechamiento
    let columnas = 8;
    let total_marcos = gestor.marcos_ram.len();
    let filas = (total_marcos as f64 / columnas as f64).ceil() as usize;

    if filas == 0 {
        return;
    }

    let grid = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2);
            filas.min(area_interna.height as usize / 2)
        ])
        .split(area_interna);

    for (i, marco) in gestor.marcos_ram.iter().enumerate() {
        let fila_idx = i / columnas;
        if fila_idx >= grid.len() {
            break;
        }

        let fila_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, columnas as u32); columnas])
            .split(grid[fila_idx]);

        let col_idx = i % columnas;
        if col_idx >= fila_layout.len() {
            continue;
        }

        let (texto, estilo) = if let Some(pid) = marco.proceso_id {
            let color = gestor
                .procesos
                .iter()
                .find(|p| p.pid == pid)
                .map(|p| {
                    // Colores más profesionales y sutiles
                    let base = (p.color.0 as u16 + p.color.1 as u16 + p.color.2 as u16) / 3;
                    if base > 180 {
                        Color::Cyan
                    } else if base > 100 {
                        Color::Blue
                    } else {
                        Color::Magenta
                    }
                })
                .unwrap_or(Color::White);

            (
                format!("P{:02}", pid),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )
        } else {
            ("--".to_string(), Style::default().fg(Color::DarkGray))
        };

        let celda = Paragraph::new(texto)
            .alignment(Alignment::Center)
            .style(estilo);

        f.render_widget(celda, fila_layout[col_idx]);
    }
}

/// Panel de información lateral
fn dibujar_panel_info(f: &mut Frame, gestor: &GestorMemoria, area: ratatui::layout::Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Metrics
            Constraint::Min(4),    // Process list
        ])
        .split(area);

    // Métricas detalladas
    let metricas = vec![
        format!("Accesos Totales: {}", gestor.accesos_totales),
        format!("Fallos de Página: {}", gestor.fallos_pagina),
        format!("Tasa de Fallos: {:.2}%", gestor.tasa_fallos()),
        format!("Swaps Realizados: {}", gestor.swaps_realizados),
    ];

    let bloque_metricas = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(" Métricas de Rendimiento ")
        .style(Style::default().fg(Color::Yellow));

    let parrafo_metricas = Paragraph::new(metricas.join("\n"))
        .block(bloque_metricas)
        .style(Style::default().fg(Color::White));

    f.render_widget(parrafo_metricas, layout[0]);

    // Lista de procesos activos
    let procesos: Vec<ListItem> = gestor
        .procesos
        .iter()
        .map(|p| {
            let paginas_en_ram = p.paginas.iter().filter(|pg| pg.marco_id.is_some()).count();
            let texto = format!(
                "PID {:02} | {}KB | {}/{} pgs",
                p.pid,
                p.tamaño_kb,
                paginas_en_ram,
                p.paginas.len()
            );
            ListItem::new(texto).style(Style::default().fg(Color::White))
        })
        .collect();

    let bloque_procesos = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(format!(" Procesos Activos ({}) ", gestor.procesos.len()))
        .style(Style::default().fg(Color::Cyan));

    let lista = List::new(procesos).block(bloque_procesos);
    f.render_widget(lista, layout[1]);
}

/// Logs del sistema sin emojis
fn dibujar_logs(f: &mut Frame, gestor: &GestorMemoria, area: ratatui::layout::Rect) {
    let logs: Vec<ListItem> = gestor
        .logs
        .iter()
        .rev()
        .take(6)
        .map(|msg| {
            let estilo = if msg.contains("ERROR") || msg.contains("lleno") {
                Style::default().fg(Color::Red)
            } else if msg.contains("NUEVO") || msg.contains("creado") {
                Style::default().fg(Color::Green)
            } else if msg.contains("SWAP") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(msg.as_str()).style(estilo)
        })
        .collect();

    let bloque = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(" Registro del Sistema ")
        .style(Style::default().fg(Color::White));

    let lista = List::new(logs).block(bloque);
    f.render_widget(lista, area);
}

/// Panel de Swap
fn dibujar_swap(f: &mut Frame, gestor: &GestorMemoria, area: ratatui::layout::Rect) {
    let swap_items: Vec<ListItem> = gestor
        .cola_swap
        .iter()
        .take(6)
        .map(|(pid, pag)| {
            ListItem::new(format!("PID {:02} Página {}", pid, pag))
                .style(Style::default().fg(Color::Magenta))
        })
        .collect();

    let bloque = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(format!(" Área de Swap ({}/50) ", gestor.cola_swap.len()))
        .style(Style::default().fg(Color::Magenta));

    let lista = List::new(swap_items).block(bloque);
    f.render_widget(lista, area);
}

/// Barra de footer con controles
fn dibujar_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let controles =
        " Q:Salir | P:Pausar | N:Nuevo Proceso | K:Matar Proceso | 1/2/3:Algoritmo | A:Cambiar Modo ";

    let footer = Paragraph::new(controles)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}

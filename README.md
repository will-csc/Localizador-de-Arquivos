# Finder in Drivers

"Finder" é uma ferramenta escrita em Rust para indexar e buscar arquivos e pastas rapidamente em unidades de disco. Ele utiliza um HashMap para armazenar as localizações dos arquivos e diretórios, permitindo buscas otimizadas e eficientes.

![image](https://github.com/user-attachments/assets/317b5035-b965-4987-aa26-199306189d9b)


## Funcionalidades
- Indexa todas as pastas e arquivos de uma unidade.
- Realiza buscas rápidas por nome de arquivo ou diretório.
- Usa cache para otimizar buscas repetidas.
- Suporte a buscas filtradas para arquivos ou pastas.

## Tecnologias Utilizadas
- **Rust**
- **sysinfo**: Para obter informações sobre as partições do sistema.
- **serde_json**: Para salvar e carregar resultados do cache.
- **lazy_static**: Para gerenciamento de cache global.
- **slint**: Para interface gráfica (opcional, caso implementado).

## Como Usar
1. Clone o repositório:
   ```sh
   git clone https://github.com/will-csc/Localizador-de-Arquivos
   cd file-finder
   ```
2. Instale as dependências:
   ```sh
   cargo build
   ```
3. Execute o programa:
   ```sh
   cargo run
   ```

## Exemplo de Uso
Para indexar um disco e buscar um arquivo:
```rust
index_drive("C:/");
let resultado = find_in("documento.txt", false, true);
println!("Resultados: {}", resultado);
```

## Contribuição
Vale resslatar que esté um projeto iniciante, então há melhorias a serem feitas. Então se quiser contribuir, sinta-se à vontade para abrir um pull request ou relatar problemas na aba de issues.

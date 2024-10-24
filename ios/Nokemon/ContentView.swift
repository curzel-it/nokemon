//
//  ContentView.swift
//  Nokemon
//
//  Created by Federico Curzel on 23/10/24.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
        .onAppear {
            
            test_integration()
            
            initialize_config(
                "en",
                folderContaining(name: "1001", extension: "json", folder: "data"),
                filePath(name: "species", extension: "json", folder: "data"),
                filePath(name: "inventory", extension: "json", folder: "data"),
                saveFilePath(),
                folderContaining(name: "en", extension: "stringx", folder: "lang")
            )
            
            initialize_game(false)
            window_size_changed(400, 400, 1, 1, 1)
            update_game(0.1)
            
            fetchRenderableItems { renderableItems in
                for item in renderableItems {
                    print("Sprite Sheet ID: \(item.sprite_sheet_id)")
                    print("Texture Rect: (x: \(item.texture_rect.x), y: \(item.texture_rect.y), width: \(item.texture_rect.width), height: \(item.texture_rect.height))")
                    print("Offset: (x: \(item.offset.x), y: \(item.offset.y))")
                    print("Frame: (x: \(item.frame.x), y: \(item.frame.y), width: \(item.frame.width), height: \(item.frame.height))")
                }
            }
            
            fetchBiomeTiles { tiles in
                for y in 0..<tiles.count {
                    for x in 0..<tiles[y].count {
                        print("y \(y) x \(x) \(tiles[y][x].tile_type)")
                    }
                }
            }
        }
    }
}

@frozen
public struct IntRect {
    public var x: Float
    public var y: Float
    public var width: Float
    public var height: Float
}

@frozen
public struct Vector2d {
    public var x: Float
    public var y: Float
}

@frozen
public struct RenderableItem {
    public var sprite_sheet_id: UInt32
    public var texture_rect: IntRect
    public var offset: Vector2d
    public var frame: IntRect
}

@_silgen_name("renderables")
func renderables(_ length: UnsafeMutablePointer<size_t>?) -> UnsafeMutablePointer<RenderableItem>?

@_silgen_name("free_renderables")
func free_renderables(_ ptr: UnsafeMutablePointer<RenderableItem>?, _ length: size_t)

func fetchRenderableItems(_ callback: @escaping ([RenderableItem]) -> Void) {
    var length: size_t = 0

    guard let ptr = renderables(&length) else {
        print("Failed to fetch renderables")
        return
    }

    let buffer = UnsafeBufferPointer<RenderableItem>(start: ptr, count: length)
    let items = Array(buffer)

    callback(items)
    free_renderables(ptr, length)
}

func filePath(name: String, extension ext: String, folder: String) -> String {
    Bundle.main.url(forResource: name, withExtension: ext, subdirectory: folder)?
        .absoluteString
        .replacingOccurrences(of: "file:///", with: "/") ?? "iOS file not found \(folder)/\(name).\(ext)"
}

func folderContaining(name: String, extension ext: String, folder: String) -> String {
    filePath(name: name, extension: ext, folder: folder)
        .replacingOccurrences(of: "/\(name).\(ext)", with: "")
}

func saveFilePath() -> String {
    let fileManager = FileManager.default
    let documentsDirectory = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
    let saveFileURL = documentsDirectory.appendingPathComponent("save.json")
    
    if !fileManager.fileExists(atPath: saveFileURL.path) {
        let defaultContents = "{\"always\": 1}".data(using: .utf8)
        fileManager.createFile(atPath: saveFileURL.path, contents: defaultContents, attributes: nil)
    }
    
    return saveFileURL.path
}

public typealias Biome = UInt32

@frozen
public struct BiomeTile {
    public var tile_type: Biome
    public var tile_up_type: Biome
    public var tile_right_type: Biome
    public var tile_down_type: Biome
    public var tile_left_type: Biome
    public var texture_offset_x: Int32
    public var texture_offset_y: Int32
}

@_silgen_name("get_biome_tiles")
func get_biome_tiles(_ out_tiles: UnsafeMutablePointer<UnsafePointer<BiomeTile>?>?,
                     _ out_len_x: UnsafeMutablePointer<size_t>?,
                     _ out_len_y: UnsafeMutablePointer<size_t>?)

@_silgen_name("free_biome_tiles")
func free_biome_tiles(_ tiles_ptr: UnsafeMutablePointer<BiomeTile>?,
                      _ len_x: size_t,
                      _ len_y: size_t)

func fetchBiomeTiles(_ callback: @escaping ([[BiomeTile]]) -> Void) {
    var tilesPtr: UnsafePointer<BiomeTile>?
    var lenX: size_t = 0
    var lenY: size_t = 0

    get_biome_tiles(&tilesPtr, &lenX, &lenY)

    guard let tilesPtr = tilesPtr else {
        print("Failed to fetch biome tiles")
        return
    }

    let totalTiles = Int(lenX * lenY)
    let buffer = UnsafeBufferPointer(start: tilesPtr, count: totalTiles)
    let tilesArray = Array(buffer)

    var tiles2D = [[BiomeTile]]()
    tiles2D.reserveCapacity(Int(lenY))

    for y in 0..<Int(lenY) {
        let startIdx = y * Int(lenX)
        let endIdx = startIdx + Int(lenX)
        let row = Array(tilesArray[startIdx..<endIdx])
        tiles2D.append(row)
    }
    
    callback(tiles2D)
    free_biome_tiles(UnsafeMutablePointer(mutating: tilesPtr), lenX, lenY)
}

public typealias Construction = UInt32

@frozen
public struct ConstructionTile {
    public var tile_type: Construction
    public var tile_up_type: Construction
    public var tile_right_type: Construction
    public var tile_down_type: Construction
    public var tile_left_type: Construction
    public var texture_source_rect: IntRect
}

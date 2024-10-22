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
        }
    }
}

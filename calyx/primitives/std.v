module std_const
  #(parameter width = 32,
    parameter val = 0)
   (input logic valid,
    output logic               ready,
    output logic [width - 1:0] out);
   assign out = val;
endmodule

module std_register
  #(parameter width = 32,
    parameter reset_val = 0)
   (input logic  [width-1:0] in,
    input logic                reset,
    input logic                valid,
    input logic                clk,
    // output
    output logic [width - 1:0] out,
    output logic               ready);

   logic [width-1:0]           register;
   always_ff @(posedge clk) begin
      if (reset) begin
         register <= reset_val;
      end else begin
         register <= in;
      end
   end

   always_comb begin
      if (valid) begin
         out = register;
         ready = 1'd1;
      end else begin
         ready = 1'd0;
      end
   end
endmodule
